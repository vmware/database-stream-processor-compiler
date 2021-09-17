#![cfg(test)]

use crate::{validation, FileId, Interner, NodeCache, Parsed, RuleCtx};
use ddlog_diagnostics::{DiagnosticConfig, FileCache, Rope};
use expect_test::{expect, Expect};
use once_cell::sync::Lazy;
use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Write},
    panic::{self, AssertUnwindSafe},
    path::Path,
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

                let (mut parsed, ret_cache) =
                    try_parse(file, &contents.to_string(), cache.take().unwrap());
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

#[track_caller]
fn check(input: &str, expected: Expect) {
    let cache = NodeCache::from_interner(INTERNER.clone());
    let file = FileId::new(INTERNER.get_or_intern_static("tests/test_file.dl"));

    let (parsed, _cache) = crate::parse_expr(file, input, cache);
    let tree = parsed.debug_tree();

    assert!(parsed.errors().is_empty());
    expected.assert_eq(&tree);
}

fn try_parse(file: FileId, text: &str, cache: NodeCache<'static>) -> (Parsed, NodeCache<'static>) {
    let result = panic::catch_unwind(AssertUnwindSafe(|| crate::parse(file, text, cache)));

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
