use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        self, BIN_EXPR, BIN_OP, BOOL, IDENT, NUMBER, NUMBER_LITERAL, PAREN_EXPR, STRING_LITERAL,
        UNARY_EXPR, UNARY_OP, VAR_REF,
    },
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

const LITERAL_TOKENS: TokenSet = token_set! {
    T![true],
    T![false],
    NUMBER,
    // TODO: Floats, strings
};

pub(super) const EXPR_RECOVERY_SET: TokenSet = token_set! {
    T![')'],
    T!['}'],
    T![;],
};

impl Parser<'_, '_> {
    // test(stmt) binary_ops
    // - 1 == 2;
    // - foo != bar;
    // - 1 * 2;
    // - 1 * 3 != 3 * 1;
    // - 1 == 2 and 5 == 10;
    // - 1 == 2 or 5 == 10;
    // - (1 == 2 or 5 == 10) and (1 == 2 and 5 == 10);
    // - 1 + 2 + 3 + 4;
    // - 1 + 2 * 3 - 4;
    // test(expr) infix_negation
    // - -10
    // test(expr) negation_has_higher_binding_power_than_infix
    // - -20 + 20
    // test(expr) parentheses_affect_precedence
    // - 5 * (2 + 1)
    pub(super) fn expr(&mut self) -> Option<CompletedMarker> {
        self.expr_inner(0)
    }

    /// The innards of [`Parser::expr()`], does all of the actual work
    ///
    /// Utilizes [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    /// to parse operator precedence
    // FIXME: Make this iterative instead of recursive
    // FIXME: This needs to be totally restructured, this fundamentally
    //        mis-structured
    fn expr_inner(&mut self, mut current_precedence: u8) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let mut lhs = match self.peek() {
            IDENT => self.identifier(VAR_REF).unwrap(),
            NUMBER | T![true] | T![false] => self.literal().unwrap(),
            operator @ (T![-] | T![!]) => self.unary_expr(operator)?,
            T!['('] => self.parentheses()?,
            T!['{'] => self.block(false)?,
            T![return] => self.ret(false)?,

            _ => return None,
        };

        // Infix operators
        while !self.at_end() {
            let precedence = match infix_precedence(dbg!(self.peek())) {
                Some(operand) => operand,
                // TODO: Error handling
                None => break,
            };

            // Eat the operator’s token.
            let operand = self.start();
            self.bump();
            operand.complete(self, BIN_OP);

            let marker = lhs.precede(self);
            let rhs_invalid = self.expr_inner(precedence).is_none();
            lhs = marker.complete(self, BIN_EXPR);

            if precedence < current_precedence {
                break;
            }
            current_precedence = precedence;

            if rhs_invalid {
                break;
            }
        }

        Some(lhs)
    }

    // test(expr) parenthesised
    // - (((((((((2 + (((((5))))))))))))))
    // test_err(expr) unclosed_paren
    // - (((100))
    fn parentheses(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        if !self.expect(T!['(']) {
            marker.abandon(self);
            return None;
        }

        let expr_invalid = self.expr().is_none();
        self.expect(T![')']);

        let complete = marker.complete(self, PAREN_EXPR);
        if expr_invalid {
            return None;
        }

        Some(complete)
    }

    // test(expr) negation
    // - --(-100)
    // test(expr) boolean_not
    // - !!(!true)
    fn unary_expr(&mut self, operator: SyntaxKind) -> Option<CompletedMarker> {
        let precedence = match unary_precedence(operator) {
            Some(operand) => operand,
            None => unreachable!(),
        };

        let marker = self.start();

        // Eat the operator’s token.
        let operand = self.start();
        self.expect(operator);
        operand.complete(self, UNARY_OP);

        let expr_invalid = self.expr_inner(precedence).is_none();

        let complete = marker.complete(self, UNARY_EXPR);
        if expr_invalid {
            return None;
        }

        Some(complete)
    }

    // test(expr) decimal_number
    // - 123
    // test(expr) hex_number
    // - 0xFFFF
    // test(expr) binary_number
    // - 0b010101
    // test(expr) bool_true
    // - true
    // test(expr) bool_false
    // - false
    pub(super) fn literal(&mut self) -> Option<CompletedMarker> {
        let literal = self.start();

        // Numbers
        if self.at(NUMBER_LITERAL) {
            self.bump();
            Some(literal.complete(self, NUMBER_LITERAL))

        // Strings
        } else if self.at(STRING_LITERAL) {
            self.bump();
            Some(literal.complete(self, STRING_LITERAL))

        // `true`
        } else if self.at(T![true]) {
            let bool = self.start();
            self.bump();
            bool.complete(self, T![true]);
            Some(literal.complete(self, BOOL))

        // `false`
        } else if self.at(T![false]) {
            let bool = self.start();
            self.bump();
            bool.complete(self, T![false]);
            Some(literal.complete(self, BOOL))

        // Non-literals
        } else {
            literal.abandon(self);
            None
        }
    }

    // TODO: Move to utils
    // test(expr) var_refs
    // - foo1245__
    // test(expr) underline_var
    // - _
    pub(super) fn identifier(&mut self, wrapper: SyntaxKind) -> Option<CompletedMarker> {
        let marker = self.start();
        match self.current() {
            IDENT => self.bump(),

            _ => {
                let error = Diagnostic::error()
                    .with_message("expected an identifier")
                    .with_label(
                        Label::primary(self.current_span()).with_message("expected an identifier"),
                    );
                self.error(error);

                marker.abandon(self);
                return None;
            }
        }

        Some(marker.complete(self, wrapper))
    }
}

fn unary_precedence(operand: SyntaxKind) -> Option<u8> {
    let precedence = match operand {
        T![-] | T![!] => 1,
        _ => return None,
    };

    Some(precedence)
}

fn infix_precedence(operand: SyntaxKind) -> Option<u8> {
    let precedence = match operand {
        T![or] => 1,
        T![and] => 2,
        T![==] | T![!=] | T![<] | T![<=] | T![>] | T![>=] => 3,
        T![|] => 4,
        T![^] => 5,
        T![&] => 6,
        T![<<] | T![>>] => 7,
        T![+] | T![-] => 8,
        T![*] | T![/] | T![%] => 9,

        _ => return None,
    };

    Some(precedence)
}
