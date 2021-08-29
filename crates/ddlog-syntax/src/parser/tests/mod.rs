#![cfg(test)]

mod expr;

use crate::{FileId, Interner, NodeCache};
use expect_test::{expect, Expect};
use once_cell::sync::Lazy;

#[track_caller]
fn check(input: &str, expected: Expect) {
    // Use a single interner for tests to reduce memory usage
    static INTERNER: Lazy<Interner> = Lazy::new(Interner::new);

    let mut cache_interner = INTERNER.clone();
    let mut cache = NodeCache::with_interner(&mut cache_interner);
    let file = FileId::new(INTERNER.get_or_intern_static("tests/test_file.dl"));

    let (root, errors) = crate::parse_expr(file, input, &mut cache);

    let mut parsed = root.debug(&*INTERNER, true);
    parsed.pop();

    assert!(errors.is_empty());
    expected.assert_eq(&parsed);
}

#[test]
fn empty_file() {
    check("", expect![[r#"ROOT@0..0"#]]);
}
