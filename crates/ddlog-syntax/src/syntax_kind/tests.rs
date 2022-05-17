#![cfg(test)]

use crate::SyntaxKind::{self, ERROR, IDENT, NUMBER_LITERAL, WHITESPACE};
use logos::Logos;

#[test]
fn syntax_kind_conversion() {
    assert_eq!(SyntaxKind::from(u16::from(ERROR)), ERROR);
    assert_eq!(SyntaxKind::from(u16::from(T![fn])), T![fn]);
}

#[track_caller]
fn check(input: &str, kind: SyntaxKind) {
    let mut lexer = SyntaxKind::lexer(input);

    assert_eq!(lexer.next(), Some(kind));
    assert_eq!(lexer.slice(), input);
}

#[test]
fn whitespace() {
    check("   ", WHITESPACE);
    check("\n", WHITESPACE);
    check("\r\n", WHITESPACE);
    check("\t\t\t", WHITESPACE);
}

#[test]
fn function_keyword() {
    check("fn", T![fn]);
}

#[test]
fn alphabetic_identifier() {
    check("abcd", IDENT);
}

#[test]
fn alphanumeric_identifier() {
    check("ab123cde456", IDENT);
}

#[test]
fn mixed_case_identifier() {
    check("ABCdef", IDENT);
}

#[test]
fn number() {
    check("123456", NUMBER_LITERAL);
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
    check("x", IDENT);
    check("_", IDENT);
}

#[test]
fn prefix_not_part_of_integer() {
    let mut lexer = SyntaxKind::lexer("+1");
    assert_eq!(lexer.next(), Some(T![+]));
    assert_eq!(lexer.next(), Some(NUMBER_LITERAL));

    let mut lexer = SyntaxKind::lexer("-1");
    assert_eq!(lexer.next(), Some(T![-]));
    assert_eq!(lexer.next(), Some(NUMBER_LITERAL));

    let mut lexer = SyntaxKind::lexer("1+1");
    assert_eq!(lexer.next(), Some(NUMBER_LITERAL));
    assert_eq!(lexer.next(), Some(T![+]));
    assert_eq!(lexer.next(), Some(NUMBER_LITERAL));

    let mut lexer = SyntaxKind::lexer("!1");
    assert_eq!(lexer.next(), Some(T![!]));
    assert_eq!(lexer.next(), Some(NUMBER_LITERAL));
}
