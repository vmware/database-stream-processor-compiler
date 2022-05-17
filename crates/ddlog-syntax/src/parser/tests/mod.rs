#![cfg(test)]

use crate::{FileId, Interner, NodeCache};
use expect_test::{expect, Expect};
use once_cell::sync::Lazy;
use tracing::subscriber;
use tracing_subscriber::EnvFilter;

#[test]
fn empty_file() {
    let _ = subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_env("DDLOG_LOG"))
            .with_test_writer()
            .finish(),
    );

    check("", expect![[r#"ROOT@0..0"#]]);
}

#[track_caller]
fn check(input: &str, expected: Expect) {
    // Use a single interner for tests to reduce memory usage
    static INTERNER: Lazy<Interner> = Lazy::new(Interner::new);

    let cache = NodeCache::from_interner(INTERNER.clone());
    let file = FileId::new(INTERNER.get_or_intern_static("tests/test_file.dl"));

    let (parsed, _cache) = crate::parse_expr(file, input, cache);
    let tree = parsed.debug_tree();

    assert!(parsed.errors().is_empty());
    expected.assert_eq(&tree);
}
