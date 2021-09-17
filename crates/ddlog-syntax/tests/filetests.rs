use ddlog_diagnostics::{DiagnosticConfig, FileCache, FileId, Interner, Rope};
use ddlog_syntax::{validation, NodeCache, Parsed, RuleCtx};
use expect_test::expect_file;
use libtest_mimic::{Arguments, Outcome, Test};
use std::{
    fmt::{self, Display},
    fs,
    panic::{self, AssertUnwindSafe},
    path::{Path, PathBuf},
    str::FromStr,
};
use tracing::subscriber;
use tracing_subscriber::EnvFilter;

const TEST_DIRS: &[&str] = &[
    "parse/pass",
    "parse/fail",
    "validation/pass",
    "validation/fail",
];

fn main() {
    let _ = subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_env("DDLOG_LOG"))
            .with_test_writer()
            .finish(),
    );

    let args = Arguments::from_args();
    let root_dir = project_root().join("ddlog-syntax/tests");
    let tests = collect_ddlog_files(&root_dir, TEST_DIRS);

    let interner = Interner::new();
    let diagnostic_config = DiagnosticConfig::new().with_color(false);

    let conclusion = libtest_mimic::run_tests(&args, tests, move |test| {
        let TestData {
            kind,
            pass: should_pass,
            validate,
            ignored,
            ref contents,
            ref expected_path,
        } = test.data;

        if ignored {
            return Outcome::Ignored;
        }

        let cache = NodeCache::from_interner(interner.clone());
        let mut file_cache = FileCache::new(interner.clone());

        // Add the file to the file cache
        let file = FileId::new(interner.get_or_intern(&test.name));
        file_cache.add_str(file, contents);

        // Attempt to parse the file
        let (parsed, _cache) = match try_parse(file, contents, kind, cache) {
            Ok(parsed) => parsed,
            // If parsing fails due to a panic, immediately fail
            Err(outcome) => return outcome,
        };
        let (root, mut errors) = parsed.into_parts();

        // If the file has validation enabled, validate the file
        if validate {
            let mut ctx = RuleCtx::new(file, Rope::from_str(contents), interner.clone());
            validation::run_validators(&root, &mut ctx);
            errors.extend(ctx.diagnostics);
        }

        if should_pass && !errors.is_empty() {
            let mut printed_errors = Vec::new();
            let num_errors = errors.len();

            for error in errors {
                error
                    .emit_to(&diagnostic_config, &mut file_cache, &mut printed_errors)
                    .expect("failed to output error");
            }

            let message = format!(
                "there should be no errors in '{}', but {} were found\n--\n{}\n--\n{}",
                test.name,
                num_errors,
                root.debug(&interner, true),
                String::from_utf8(printed_errors).expect("invalid utf8 in errors"),
            );

            return Outcome::Failed { msg: Some(message) };
        } else if !should_pass && errors.is_empty() {
            let message = format!("there should be errors in '{}'", test.name);
            return Outcome::Failed { msg: Some(message) };
        }

        let mut actual_output = root.debug(&interner, true);
        if !should_pass {
            let mut buffer = Vec::new();

            for diagnostic in errors {
                diagnostic
                    .emit_to(&diagnostic_config, &mut file_cache, &mut buffer)
                    .expect("failed to emit diagnostic");

                actual_output.push_str(&format!(
                    "--\n{}",
                    std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
                ));

                buffer.clear();
            }

            actual_output.push_str(&format!("--\n{}", contents));
        }

        let expected = panic::catch_unwind(|| {
            expect_file![expected_path].assert_eq(&actual_output);
        });

        if expected.is_err() {
            Outcome::Failed {
                msg: Some(format!(
                    "{}'s output didn't match its expected output",
                    test.name,
                )),
            }
        } else {
            Outcome::Passed
        }
    });

    if !args.quiet {
        println!(
            "{} passed, {} failed, {} ignored",
            conclusion.num_passed(),
            conclusion.num_failed(),
            conclusion.num_ignored(),
        );
    }

    conclusion.exit_if_failed();
}

fn try_parse(
    file: FileId,
    text: &str,
    kind: TestKind,
    cache: NodeCache<'static>,
) -> Result<(Parsed, NodeCache<'static>), Outcome> {
    let interner = cache.interner().clone();

    panic::catch_unwind(AssertUnwindSafe(|| match kind {
        TestKind::Item => ddlog_syntax::parse(file, text, cache),
        TestKind::Stmt => ddlog_syntax::parse_stmt(file, text, cache),
        TestKind::Expr => ddlog_syntax::parse_expr(file, text, cache),
    }))
    .map_err(|error| Outcome::Failed {
        msg: Some(format!(
            "panicked while trying to parse '{}': {:?}",
            file.to_str(&interner),
            error
        )),
    })
}

fn project_root() -> &'static Path {
    const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

    Path::new(MANIFEST)
        .parent()
        .expect("CARGO_MANIFEST_DIR has no parent??")
}

fn collect_ddlog_files(root_dir: &Path, paths: &[&str]) -> Vec<Test<TestData>> {
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

            let (mut kind, mut validate, mut pass, mut ignored) =
                (TestKind::Item, false, true, false);
            for attr in header.split(' ') {
                if let Some(k) = attr.strip_prefix("kind:") {
                    kind = k.parse().expect("invalid test kind");
                } else if let Some(v) = attr.strip_prefix("validate:") {
                    match v {
                        "true" => validate = true,
                        "false" => validate = false,
                        _ => panic!("invalid `validate` setting: {:?}", v),
                    }
                } else if let Some(p) = attr.strip_prefix("pass:") {
                    match p {
                        "true" => pass = true,
                        "false" => pass = false,
                        _ => panic!("invalid `pass` setting: {:?}", p),
                    }
                } else if let Some(i) = attr.strip_prefix("ignore:") {
                    match i {
                        "true" => ignored = true,
                        "false" => ignored = false,
                        _ => panic!("invalid `ignore` setting: {:?}", i),
                    }
                }
            }

            // Strip the prefix off of the file's path
            let name = path
                .strip_prefix(root_dir)
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| path.display().to_string())
                .replace('\\', "/");

            // Make a readable kind for the test
            let pretty_kind = format!("{} {}", if validate { "validate" } else { "parse" }, kind);

            let contents = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read '{}': {:?}", path.display(), error));
            let expected_path = path.with_extension("rast");

            Test {
                name,
                kind: pretty_kind,
                is_ignored: false,
                is_bench: false,
                data: TestData {
                    kind,
                    pass,
                    validate,
                    ignored,
                    contents,
                    expected_path,
                },
            }
        })
        .collect()
}

#[derive(Debug)]
struct TestData {
    kind: TestKind,
    pass: bool,
    validate: bool,
    ignored: bool,
    contents: String,
    expected_path: PathBuf,
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

#[derive(Debug, Clone, Copy)]
enum TestKind {
    Item,
    Stmt,
    Expr,
}

impl Display for TestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Item => f.write_str("item"),
            Self::Stmt => f.write_str("stmt"),
            Self::Expr => f.write_str("expr"),
        }
    }
}

impl FromStr for TestKind {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match &*string.trim().to_lowercase() {
            "item" => Self::Item,
            "stmt" => Self::Stmt,
            "expr" => Self::Expr,
            kind => {
                return Err(format!(
                    "invalid test `kind` (expected one of `item`, `stmt` or `expr`): {:?}",
                    kind
                ))
            }
        })
    }
}
