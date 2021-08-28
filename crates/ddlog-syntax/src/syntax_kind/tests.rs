#![cfg(test)]

use crate::SyntaxKind;
use logos::Logos;

#[test]
fn syntax_kind_conversion() {
    assert_eq!(SyntaxKind::from(u16::from(T![error])), T![error]);
    assert_eq!(SyntaxKind::from(u16::from(T![function])), T![function]);
}

#[track_caller]
fn check(input: &str, kind: SyntaxKind) {
    let mut lexer = SyntaxKind::lexer(input);

    assert_eq!(lexer.next(), Some(kind));
    assert_eq!(lexer.slice(), input);
}

#[test]
fn whitespace() {
    check("   ", T![whitespace]);
    check("\n", T![whitespace]);
    check("\r\n", T![whitespace]);
    check("\t\t\t", T![whitespace]);
}

#[test]
fn function_keyword() {
    check("function", T![function]);
}

#[test]
fn alphabetic_identifier() {
    check("abcd", T![ident]);
}

#[test]
fn alphanumeric_identifier() {
    check("ab123cde456", T![ident]);
}

#[test]
fn mixed_case_identifier() {
    check("ABCdef", T![ident]);
}

#[test]
fn number() {
    check("123456", T![number]);
}

#[test]
fn plus() {
    check("+", T![+]);
}

#[test]
fn minus() {
    check("-", T![-]);
}

#[test]
fn star() {
    check("*", T![*]);
}

#[test]
fn slash() {
    check("/", T![/]);
}

#[test]
fn equals() {
    check("=", T![=]);
}

#[test]
fn left_brace() {
    check("{", T!['{']);
}

#[test]
fn right_brace() {
    check("}", T!['}']);
}

#[test]
fn single_char_identifier() {
    check("x", T![ident]);
    check("_", T![ident]);
}

#[test]
fn prefix_not_part_of_integer() {
    let mut lexer = SyntaxKind::lexer("+1");
    assert_eq!(lexer.next(), Some(T![+]));
    assert_eq!(lexer.next(), Some(T![number]));

    let mut lexer = SyntaxKind::lexer("-1");
    assert_eq!(lexer.next(), Some(T![-]));
    assert_eq!(lexer.next(), Some(T![number]));

    let mut lexer = SyntaxKind::lexer("1+1");
    assert_eq!(lexer.next(), Some(T![number]));
    assert_eq!(lexer.next(), Some(T![+]));
    assert_eq!(lexer.next(), Some(T![number]));

    let mut lexer = SyntaxKind::lexer("!1");
    assert_eq!(lexer.next(), Some(T![!]));
    assert_eq!(lexer.next(), Some(T![number]));
}
