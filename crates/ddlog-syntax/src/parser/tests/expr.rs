use super::{check, expect};

#[test]
fn decimal_number() {
    check(
        "123",
        expect![[r#"
            ROOT@0..3
              EXPR@0..3
                LITERAL@0..3
                  NUMBER@0..3 "123""#]],
    );
}

#[test]
fn hex_number() {
    check(
        "0xFFFF",
        expect![[r#"
            ROOT@0..6
              EXPR@0..6
                LITERAL@0..6
                  NUMBER@0..6 "0xFFFF""#]],
    );
}

#[test]
fn binary_number() {
    check(
        "0b010101",
        expect![[r#"
            ROOT@0..8
              EXPR@0..8
                LITERAL@0..8
                  NUMBER@0..8 "0b010101""#]],
    );
}

#[test]
fn ident() {
    check(
        "counter",
        expect![[r#"
            ROOT@0..7
              EXPR@0..7
                VAR_REF@0..7
                  IDENT@0..7 "counter""#]],
    );
}

#[test]
fn simple_binary_expression() {
    check(
        "1+2",
        expect![[r#"
            ROOT@0..3
              EXPR@0..3
                BIN_EXPR@0..3
                  LITERAL@0..1
                    NUMBER@0..1 "1"
                  BIN_OP@1..2
                    PLUS@1..2 "+"
                  LITERAL@2..3
                    NUMBER@2..3 "2""#]],
    );
}

#[test]
fn left_associative_binary_expression() {
    check(
        "1+2+3+4",
        expect![[r#"
            ROOT@0..7
              EXPR@0..7
                BIN_EXPR@0..7
                  LITERAL@0..1
                    NUMBER@0..1 "1"
                  BIN_OP@1..2
                    PLUS@1..2 "+"
                  BIN_EXPR@2..7
                    LITERAL@2..3
                      NUMBER@2..3 "2"
                    BIN_OP@3..4
                      PLUS@3..4 "+"
                    BIN_EXPR@4..7
                      LITERAL@4..5
                        NUMBER@4..5 "3"
                      BIN_OP@5..6
                        PLUS@5..6 "+"
                      LITERAL@6..7
                        NUMBER@6..7 "4""#]],
    );
}

#[test]
fn binary_expression_with_mixed_binding_power() {
    check(
        "1+2*3-4",
        expect![[r#"
            ROOT@0..7
              EXPR@0..7
                BIN_EXPR@0..7
                  BIN_EXPR@0..5
                    LITERAL@0..1
                      NUMBER@0..1 "1"
                    BIN_OP@1..2
                      PLUS@1..2 "+"
                    BIN_EXPR@2..5
                      LITERAL@2..3
                        NUMBER@2..3 "2"
                      BIN_OP@3..4
                        STAR@3..4 "*"
                      LITERAL@4..5
                        NUMBER@4..5 "3"
                  BIN_OP@5..6
                    MINUS@5..6 "-"
                  LITERAL@6..7
                    NUMBER@6..7 "4""#]],
    );
}

#[test]
fn negation() {
    check(
        "-10",
        expect![[r#"
            ROOT@0..3
              EXPR@0..3
                UNARY_EXPR@0..3
                  UNARY_OP@0..1
                    MINUS@0..1 "-"
                  LITERAL@1..3
                    NUMBER@1..3 "10""#]],
    );
}

#[test]
fn negation_has_higher_binding_power_than_infix() {
    check(
        "-20+20",
        expect![[r#"
            ROOT@0..6
              EXPR@0..6
                UNARY_EXPR@0..6
                  UNARY_OP@0..1
                    MINUS@0..1 "-"
                  BIN_EXPR@1..6
                    LITERAL@1..3
                      NUMBER@1..3 "20"
                    BIN_OP@3..4
                      PLUS@3..4 "+"
                    LITERAL@4..6
                      NUMBER@4..6 "20""#]],
    );
}

#[test]
fn nested_parentheses() {
    check(
        "((((((10))))))",
        expect![[r#"
            ROOT@0..14
              EXPR@0..14
                PAREN_EXPR@0..14
                  L_PAREN@0..1 "("
                  EXPR@1..13
                    PAREN_EXPR@1..13
                      L_PAREN@1..2 "("
                      EXPR@2..12
                        PAREN_EXPR@2..12
                          L_PAREN@2..3 "("
                          EXPR@3..11
                            PAREN_EXPR@3..11
                              L_PAREN@3..4 "("
                              EXPR@4..10
                                PAREN_EXPR@4..10
                                  L_PAREN@4..5 "("
                                  EXPR@5..9
                                    PAREN_EXPR@5..9
                                      L_PAREN@5..6 "("
                                      EXPR@6..8
                                        LITERAL@6..8
                                          NUMBER@6..8 "10"
                                      R_PAREN@8..9 ")"
                                  R_PAREN@9..10 ")"
                              R_PAREN@10..11 ")"
                          R_PAREN@11..12 ")"
                      R_PAREN@12..13 ")"
                  R_PAREN@13..14 ")""#]],
    );
}

#[test]
fn parentheses_affect_precedence() {
    check(
        "5*(2+1)",
        expect![[r#"
            ROOT@0..7
              EXPR@0..7
                BIN_EXPR@0..7
                  LITERAL@0..1
                    NUMBER@0..1 "5"
                  BIN_OP@1..2
                    STAR@1..2 "*"
                  PAREN_EXPR@2..7
                    L_PAREN@2..3 "("
                    EXPR@3..6
                      BIN_EXPR@3..6
                        LITERAL@3..4
                          NUMBER@3..4 "2"
                        BIN_OP@4..5
                          PLUS@4..5 "+"
                        LITERAL@5..6
                          NUMBER@5..6 "1"
                    R_PAREN@6..7 ")""#]],
    );
}
