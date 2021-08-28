use super::{check, expect};

#[test]
fn decimal_number() {
    check(
        "123",
        expect![[r#"
            Root@0..3
              Literal@0..3
                Number@0..3 "123""#]],
    );
}

#[test]
fn hex_number() {
    check(
        "0xFFFF",
        expect![[r#"
            Root@0..6
              Literal@0..6
                Number@0..6 "0xFFFF""#]],
    );
}

#[test]
fn binary_number() {
    check(
        "0b010101",
        expect![[r#"
            Root@0..8
              Literal@0..8
                Number@0..8 "0b010101""#]],
    );
}

#[test]
fn ident() {
    check(
        "counter",
        expect![[r#"
            Root@0..7
              VarRef@0..7
                Ident@0..7 "counter""#]],
    );
}

#[test]
fn simple_binary_expression() {
    check(
        "1+2",
        expect![[r#"
            Root@0..3
              BinaryExpr@0..3
                Literal@0..1
                  Number@0..1 "1"
                Plus@1..2 "+"
                Literal@2..3
                  Number@2..3 "2""#]],
    );
}

#[test]
fn left_associative_binary_expression() {
    check(
        "1+2+3+4",
        expect![[r#"
            Root@0..7
              BinaryExpr@0..7
                BinaryExpr@0..5
                  BinaryExpr@0..3
                    Literal@0..1
                      Number@0..1 "1"
                    Plus@1..2 "+"
                    Literal@2..3
                      Number@2..3 "2"
                  Plus@3..4 "+"
                  Literal@4..5
                    Number@4..5 "3"
                Plus@5..6 "+"
                Literal@6..7
                  Number@6..7 "4""#]],
    );
}

#[test]
fn binary_expression_with_mixed_binding_power() {
    check(
        "1+2*3-4",
        expect![[r#"
            Root@0..7
              BinaryExpr@0..7
                BinaryExpr@0..5
                  Literal@0..1
                    Number@0..1 "1"
                  Plus@1..2 "+"
                  BinaryExpr@2..5
                    Literal@2..3
                      Number@2..3 "2"
                    Star@3..4 "*"
                    Literal@4..5
                      Number@4..5 "3"
                Minus@5..6 "-"
                Literal@6..7
                  Number@6..7 "4""#]],
    );
}

#[test]
fn negation() {
    check(
        "-10",
        expect![[r#"
            Root@0..3
              PrefixExpr@0..3
                Minus@0..1 "-"
                Literal@1..3
                  Number@1..3 "10""#]],
    );
}

#[test]
fn negation_has_higher_binding_power_than_infix() {
    check(
        "-20+20",
        expect![[r#"
            Root@0..6
              BinaryExpr@0..6
                PrefixExpr@0..3
                  Minus@0..1 "-"
                  Literal@1..3
                    Number@1..3 "20"
                Plus@3..4 "+"
                Literal@4..6
                  Number@4..6 "20""#]],
    );
}

#[test]
fn nested_parentheses() {
    check(
        "((((((10))))))",
        expect![[r#"
            Root@0..14
              ParenExpr@0..14
                LParen@0..1 "("
                ParenExpr@1..13
                  LParen@1..2 "("
                  ParenExpr@2..12
                    LParen@2..3 "("
                    ParenExpr@3..11
                      LParen@3..4 "("
                      ParenExpr@4..10
                        LParen@4..5 "("
                        ParenExpr@5..9
                          LParen@5..6 "("
                          Literal@6..8
                            Number@6..8 "10"
                          RParen@8..9 ")"
                        RParen@9..10 ")"
                      RParen@10..11 ")"
                    RParen@11..12 ")"
                  RParen@12..13 ")"
                RParen@13..14 ")""#]],
    );
}

#[test]
fn parentheses_affect_precedence() {
    check(
        "5*(2+1)",
        expect![[r#"
            Root@0..7
              BinaryExpr@0..7
                Literal@0..1
                  Number@0..1 "5"
                Star@1..2 "*"
                ParenExpr@2..7
                  LParen@2..3 "("
                  BinaryExpr@3..6
                    Literal@3..4
                      Number@3..4 "2"
                    Plus@4..5 "+"
                    Literal@5..6
                      Number@5..6 "1"
                  RParen@6..7 ")""#]],
    );
}
