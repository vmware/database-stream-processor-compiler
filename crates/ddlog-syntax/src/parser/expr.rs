use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        self, BIN_EXPR, BIN_OP, BLOCK, BOOL, EOF, IDENT, LITERAL, NUMBER, PAREN_EXPR, UNARY_EXPR,
        UNARY_OP, VAR_REF,
    },
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

pub(super) const EXPR_RECOVERY_SET: TokenSet = token_set! {
    T![')'],
    T!['}'],
    T![;],
};

impl Parser<'_, '_> {
    #[must_use]
    pub(super) fn expr(&mut self) -> Option<CompletedMarker> {
        self.expr_inner(0)
    }

    /// The innards of [`Parser::expr()`], does all of the actual work
    ///
    /// Utilizes [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    /// to parse operator precedence
    // FIXME: Make this iterative instead of recursive
    #[must_use]
    fn expr_inner(&mut self, mut current_precedence: u8) -> Option<CompletedMarker> {
        let mut lhs = match self.peek() {
            // Identifiers
            ident @ IDENT => {
                let marker = self.start();
                self.expect(ident);

                marker.complete(self, VAR_REF)
            }

            // Literals
            literal @ (NUMBER | BOOL) => {
                let marker = self.start();
                self.expect(literal);

                marker.complete(self, LITERAL)
            }

            // Negation
            operator @ (T![-] | T![!]) => {
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

                complete
            }

            // Parentheses
            T!['('] => {
                let marker = self.start();

                self.expect(T!['(']);
                let expr_invalid = self.expr().is_none();
                self.expect(T![')']);

                let complete = marker.complete(self, PAREN_EXPR);
                if expr_invalid {
                    return None;
                }

                complete
            }

            // Blocks
            T!['{'] => self.block(token_set! { T!['}'] })?,

            _ => return None,
        };

        // Infix operators
        while !self.at(EOF) {
            let precedence = match infix_precedence(self.peek()) {
                Some(operand) => operand,
                // TODO: Error handling
                None => {
                    // let error = Diagnostic::error()
                    //     .with_message("expected an infix expression")
                    //     .with_label(Label::primary(self.current_span()).with_message(format!(
                    //         "expected an infix operator, got '{}'",
                    //         self.current_text(),
                    //     )));
                    //
                    // let current_set = self.recovery_set;
                    // self.recovery_set = current_set.union(EXPR_RECOVERY_SET);
                    // self.error(error);
                    // self.recovery_set = current_set;

                    // Return a precedence level of zero as a dummy
                    // infix operand
                    break;
                }
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

    // TODO: Move to utils
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

    pub(super) fn block(&mut self, recovery_set: TokenSet) -> Option<CompletedMarker> {
        let previous = self.recovery_set;
        self.recovery_set = previous.union(recovery_set);

        let block = self.start();
        self.expect(T!['{']);

        // FIXME: Statements, not expressions
        let mut did_error = false;
        while !self.at(T!['}']) && !self.at_end() {
            if self.expr().is_none() {
                did_error = true;
            }
        }

        self.expect(T!['}']);
        let marker = block.complete(self, BLOCK);

        self.recovery_set = previous;

        if did_error {
            None
        } else {
            Some(marker)
        }
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
