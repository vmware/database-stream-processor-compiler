use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        self, BIN_EXPR, BIN_OP, EXPR, IDENT, LITERAL, NUMBER, PAREN_EXPR, UNARY_EXPR, UNARY_OP,
        VAR_REF,
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
    pub(super) fn expr(&mut self) -> Option<CompletedMarker> {
        let expr = self.start();
        // TODO: Should we abandon the marker if `.expr_inner()` fails?
        self.expr_inner(0);
        let marker = expr.complete(self, EXPR);

        Some(marker)
    }

    /// The innards of [`Parser::expr()`], does all of the actual work
    ///
    /// Utilizes [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    /// to parse operator precedence
    // FIXME: Make this iterative instead of recursive
    fn expr_inner(&mut self, mut current_precedence: u8) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let mut lhs = match self.peek() {
            IDENT => self.identifier(VAR_REF).unwrap(),

            NUMBER | T![true] | T![false] => self.literal().unwrap(),

            operator @ (T![-] | T![!]) => self.unary_expr(operator)?,

            T!['('] => self.parentheses()?,

            T!['{'] => self.block()?,

            T![return] => self.ret()?,

            _ => return None,
        };

        // Infix operators
        while !self.at_end() {
            let precedence = match infix_precedence(self.peek()) {
                Some(operand) => operand,
                // TODO: Error handling
                None => break,
            };

            if precedence < current_precedence {
                break;
            }
            current_precedence = precedence;

            // Eat the operator’s token.
            let operand = self.start();
            self.bump();
            operand.complete(self, BIN_OP);

            let marker = lhs.precede(self);
            let rhs_invalid = self.expr_inner(current_precedence).is_none();
            lhs = marker.complete(self, BIN_EXPR);

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
        if !self.at_set(LITERAL_TOKENS) {
            return None;
        }

        let literal = self.start();
        self.bump();
        Some(literal.complete(self, LITERAL))
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
