#![cfg(test)]

mod expr;

use crate::{FileId, Interner, NodeCache, Parsed, SyntaxNode};
use ddlog_diagnostics::{Diagnostic, DiagnosticConfig, FileCache};
use expect_test::{expect, expect_file, Expect};
use once_cell::sync::Lazy;
use std::{
    fs,
    io::{self, Write},
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
};

// Use a single interner for tests to reduce memory usage
static INTERNER: Lazy<Interner> = Lazy::new(Interner::new);

#[track_caller]
fn check(input: &str, expected: Expect) {
    let cache = NodeCache::from_interner(INTERNER.clone());
    let file = FileId::new(INTERNER.get_or_intern_static("tests/test_file.dl"));

    let (parsed, _cache) = crate::parse_expr(file, input, cache);
    let tree = parsed.debug_tree();

    assert!(parsed.errors().is_empty());
    expected.assert_eq(&tree);
}

#[test]
fn empty_file() {
    check("", expect![[r#"ROOT@0..0"#]]);
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
        TestKind::Stmt => todo!("implement statement tests"),
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

#[test]
fn parser_tests() {
    let mut cache = Some(NodeCache::from_interner(INTERNER.clone()));
    let mut file_cache = FileCache::new(INTERNER.clone());
    let diagnostic_config = DiagnosticConfig::new().with_color(false);

    dir_tests(
        &test_data_dir(),
        &["inline/pass"],
        "rast",
        |text, path, kind| {
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

            let (parsed, ret_cache) = try_parse(file, text, kind, cache.take().unwrap());
            cache = Some(ret_cache);

            assert_errors_are_absent(
                parsed.errors(),
                path,
                parsed.root(),
                &diagnostic_config,
                &mut file_cache,
            );

            parsed.debug_tree()
        },
    );

    dir_tests(
        &test_data_dir(),
        &["inline/fail"],
        "rast",
        |text, path, kind| {
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

            let (parsed, ret_cache) = try_parse(file, text, kind, cache.take().unwrap());
            cache = Some(ret_cache);

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

fn dir_tests<F>(test_data_dir: &Path, paths: &[&str], outfile_extension: &str, mut func: F)
where
    F: FnMut(&str, &Path, TestKind) -> String,
{
    for (path, input_code, kind) in collect_ddlog_files(test_data_dir, paths) {
        eprint!("running '{}'... ", path.display());

        let actual = func(&input_code, &path, kind);
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

fn collect_ddlog_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String, TestKind)> {
    paths
        .iter()
        .flat_map(|path| {
            let path = root_dir.join(path);
            ddlog_files_in_dir(&path).into_iter()
        })
        .map(|path| {
            let text = fs::read_to_string(&path)
                .unwrap_or_else(|_| format!("could not read ddlog file '{}'", path.display()));

            let kind = if text.starts_with("// kind:item") {
                TestKind::Item
            } else if text.starts_with("// kind:stmt") {
                TestKind::Stmt
            } else if text.starts_with("// kind:expr") {
                TestKind::Expr
            } else {
                panic!("test had no kind '{}'", path.display())
            };

            (path, text, kind)
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
