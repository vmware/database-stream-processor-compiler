#![cfg(test)]

mod expr;

use crate::{ast::AstNode, validation, FileId, Interner, NodeCache, Parsed, RuleCtx, SyntaxNode};
use ddlog_diagnostics::{Diagnostic, DiagnosticConfig, FileCache, Rope};
use expect_test::{expect, expect_file, Expect};
use once_cell::sync::Lazy;
use std::{
    env,
    ffi::OsStr,
    fmt::Debug,
    fs,
    io::{self, Write},
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
};
use tracing::subscriber;
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;

// Use a single interner for tests to reduce memory usage
static INTERNER: Lazy<Interner> = Lazy::new(Interner::new);

fn test_logger() {
    let _ = subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_env("DDLOG_LOG"))
            .with_test_writer()
            .finish(),
    );
}

#[test]
fn empty_file() {
    test_logger();

    check("", expect![[r#"ROOT@0..0"#]]);
}

#[test]
#[ignore]
fn ddlog_home() {
    test_logger();

    if let Ok(ddlog_home) = env::var("DDLOG_HOME") {
        let ddlog_home = Path::new(&ddlog_home).join("lib");

        if ddlog_home.exists() {
            let ddlog_libs = WalkDir::new(&ddlog_home)
                .into_iter()
                .flatten()
                // Filter out anything that's not a `.dl` file
                .filter(|entry| {
                    entry.file_type().is_file()
                        && matches!(entry.path().extension().and_then(OsStr::to_str), Some("dl"),)
                });

            let mut file_cache = FileCache::new(INTERNER.clone());
            let diagnostic_config = DiagnosticConfig::default();
            let mut cache = Some(NodeCache::from_interner(INTERNER.clone()));

            let files: Vec<_> = ddlog_libs
                .map(|file| {
                    let uri = format!("file:{}", file.path().canonicalize().unwrap().display(),);
                    let file_id = FileId::new(INTERNER.get_or_intern(&uri));

                    let source = Rope::from(fs::read_to_string(file.path()).unwrap());
                    file_cache.add(file_id, source.clone());

                    (file_id, source)
                })
                .collect();

            let stdout = io::stdout();
            let mut stdout = stdout.lock();

            for (file, contents) in files {
                writeln!(&mut stdout, "parsing '{}'", file.to_str(&*INTERNER)).unwrap();

                let (mut parsed, ret_cache) = try_parse(
                    file,
                    &contents.to_string(),
                    TestKind::Item,
                    cache.take().unwrap(),
                );
                cache = Some(ret_cache);

                let mut ctx = RuleCtx::new(file, contents, INTERNER.clone());
                validation::run_validators(&parsed.root, &mut ctx);
                parsed.errors.extend(ctx.diagnostics);

                write!(&mut stdout, "{}", parsed.root.debug(&*INTERNER, true))
                    .expect("failed to output error");

                for error in parsed.errors {
                    error
                        .emit_to(&diagnostic_config, &mut file_cache, &mut stdout)
                        .expect("failed to output error");
                }
            }
        }
    }
}

#[test]
fn parser_tests() {
    test_logger();

    let mut cache = Some(NodeCache::from_interner(INTERNER.clone()));
    let mut file_cache = FileCache::new(INTERNER.clone());
    let diagnostic_config = DiagnosticConfig::new().with_color(false);

    dir_tests(
        &test_data_dir(),
        &["parse/pass", "validation/pass"],
        "rast",
        |text, path, kind, validate| {
            let file = FileId::new(
                INTERNER.get_or_intern(
                    &path
                        .strip_prefix(project_root().join("ddlog-syntax/tests"))
                        .unwrap_or(path)
                        .to_str()
                        .unwrap()
                        .replace("/", "\\"),
                ),
            );
            file_cache.add_str(file, text);

            let (mut parsed, ret_cache) = try_parse(file, text, kind, cache.take().unwrap());
            cache = Some(ret_cache);

            if validate {
                let mut ctx = RuleCtx::new(file, Rope::from_str(text), INTERNER.clone());
                validation::run_validators(&parsed.root, &mut ctx);
                parsed.errors.extend(ctx.diagnostics);
            }

            assert_errors_are_absent(
                parsed.errors(),
                path,
                parsed.root().syntax(),
                &diagnostic_config,
                &mut file_cache,
            );

            let mut tree = parsed.debug_tree();
            tree.push('\n');
            tree
        },
    );

    dir_tests(
        &test_data_dir(),
        &["parse/fail", "validation/fail"],
        "rast",
        |text, path, kind, validate| {
            let file = FileId::new(
                INTERNER.get_or_intern(
                    &path
                        .strip_prefix(project_root().join("ddlog-syntax/tests"))
                        .unwrap_or(path)
                        .to_str()
                        .unwrap()
                        .replace("\\", "/"),
                ),
            );
            file_cache.add_str(file, text);

            let (mut parsed, ret_cache) = try_parse(file, text, kind, cache.take().unwrap());
            cache = Some(ret_cache);

            if validate {
                let mut ctx = RuleCtx::new(file, Rope::from_str(text), INTERNER.clone());
                validation::run_validators(&parsed.root, &mut ctx);
                parsed.errors.extend(ctx.diagnostics);
            }

            assert_errors_are_present(parsed.errors(), path);

            let mut result = parsed.debug_tree();
            result.push('\n');

            let mut buffer = Vec::new();

            for diagnostic in parsed.into_errors() {
                diagnostic
                    .emit_to(&diagnostic_config, &mut file_cache, &mut buffer)
                    .expect("failed to emit diagnostic");

                result.push_str(&format!(
                    "--\n{}",
                    std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
                ));
                buffer.clear();
            }
            result.push_str(&format!("--\n{}", text));
            result
        },
    );
}

#[track_caller]
fn check(input: &str, expected: Expect) {
    let cache = NodeCache::from_interner(INTERNER.clone());
    let file = FileId::new(INTERNER.get_or_intern_static("tests/test_file.dl"));

    let (parsed, _cache) = crate::parse_expr(file, input, cache);
    let tree = parsed.debug_tree();

    assert!(parsed.errors().is_empty());
    expected.assert_eq(&tree);
}

fn test_data_dir() -> PathBuf {
    project_root().join("ddlog-syntax/tests")
}

fn try_parse(
    file: FileId,
    text: &str,
    kind: TestKind,
    cache: NodeCache<'static>,
) -> (Parsed, NodeCache<'static>) {
    let result = panic::catch_unwind(AssertUnwindSafe(|| match kind {
        TestKind::Item => crate::parse(file, text, cache),
        TestKind::Stmt => crate::parse_stmt(file, text, cache),
        TestKind::Expr => crate::parse_expr(file, text, cache),
    }));

    match result {
        Ok(parsed) => parsed,
        Err(panic) => {
            eprintln!(
                "panicked while trying to parse '{}'",
                file.to_str(&INTERNER),
            );
            panic::resume_unwind(panic);
        }
    }
}

fn dir_tests<F>(test_data_dir: &Path, paths: &[&str], outfile_extension: &str, mut func: F)
where
    F: FnMut(&str, &Path, TestKind, bool) -> String,
{
    for (path, input_code, kind, validate) in collect_ddlog_files(test_data_dir, paths) {
        eprint!("running '{}'... ", path.display());

        let actual = func(&input_code, &path, kind, validate);
        let path = path.with_extension(outfile_extension);

        expect_file![path].assert_eq(&actual);

        eprintln!("ok!");
    }
}

fn project_root() -> &'static Path {
    const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

    Path::new(MANIFEST)
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent??")
}

fn collect_ddlog_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String, TestKind, bool)> {
    paths
        .iter()
        .flat_map(|path| {
            let path = root_dir.join(path);
            ddlog_files_in_dir(&path).into_iter()
        })
        .map(|path| {
            let text = fs::read_to_string(&path)
                .unwrap_or_else(|_| panic!("could not read ddlog file '{}'", path.display()));

            let header = text.lines().next().expect("expected a test header");
            assert!(header.starts_with("//"), "header must start with `//`");

            let (mut kind, mut validate) = (TestKind::Item, false);
            for attr in header.split(' ') {
                if let Some(k) = attr.strip_prefix("kind:") {
                    match k {
                        "item" => kind = TestKind::Item,
                        "stmt" => kind = TestKind::Stmt,
                        "expr" => kind = TestKind::Expr,
                        _ => panic!("invalid `kind` setting: {:?}", k),
                    }
                } else if let Some(v) = attr.strip_prefix("validate:") {
                    match v {
                        "true" => validate = true,
                        "false" => validate = false,
                        _ => panic!("invalid `validate` setting: {:?}", v),
                    }
                }
            }

            (path, text, kind, validate)
        })
        .collect()
}

fn ddlog_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let dir = fs::read_dir(&dir)
        .unwrap_or_else(|_| panic!("failed to read test directory '{}'", dir.display()));
    for file in dir {
        let file = file.unwrap();
        let path = file.path();

        if path.extension().unwrap_or_default() == "dl" {
            files.push(path);
        }
    }

    files.sort();
    files
}

fn assert_errors_are_present(errors: &[Diagnostic], path: &Path) {
    assert!(
        !errors.is_empty(),
        "there should be errors in the file '{}'",
        path.display(),
    );
}

fn assert_errors_are_absent(
    errors: &[Diagnostic],
    path: &Path,
    root: &SyntaxNode,
    config: &DiagnosticConfig,
    cache: &mut FileCache,
) {
    if !errors.is_empty() {
        {
            let stderr = io::stderr();
            let mut stderr = stderr.lock();

            write!(&mut stderr, "{}", root.debug(&*INTERNER, true))
                .expect("failed to output error");

            for error in errors.iter().cloned() {
                error
                    .emit_to(config, cache, &mut stderr)
                    .expect("failed to output error");
            }
        }

        panic!(
            "there should be no errors in the file '{}', but {} were found",
            path.display(),
            errors.len(),
        );
    }
}

#[derive(Debug, Clone, Copy)]
enum TestKind {
    Item,
    Stmt,
    Expr,
}
