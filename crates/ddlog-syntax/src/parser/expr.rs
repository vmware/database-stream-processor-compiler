use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        self, BIN_EXPR, BIN_OP, BLOCK, EOF, IDENT, LITERAL, NUMBER, PAREN_EXPR, UNARY_EXPR,
        UNARY_OP, VAR_REF,
    },
    TokenSet,
};
use ariadne::{Label, Report, ReportKind};

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
            // Identifiers are atomic
            IDENT => {
                let marker = self.start();
                self.bump();

                marker.complete(self, VAR_REF)
            }

            // Numbers are atomic
            NUMBER => {
                let marker = self.start();
                self.bump();

                marker.complete(self, LITERAL)
            }

            // Negation
            operand @ (T![-] | T![!]) => {
                let precedence = match unary_precedence(operand) {
                    Some(operand) => operand,
                    None => unreachable!(),
                };

                let marker = self.start();

                // Eat the operator’s token.
                let operand = self.start();
                self.bump();
                operand.complete(self, UNARY_OP);

                self.expr_inner(precedence);
                marker.complete(self, UNARY_EXPR)
            }

            // Parentheses
            T!['('] => {
                let marker = self.start();

                // Eat the `(`
                self.bump();
                self.expr();

                // TODO: Error handling
                assert_eq!(self.peek(), T![')']);
                self.bump();

                marker.complete(self, PAREN_EXPR)
            }

            // TODO: Error handling
            _ => return None,
        };

        // Infix operators
        while !self.at(EOF) {
            let precedence = match infix_precedence(self.peek()) {
                Some(operand) => operand,
                // TODO: Error handling
                None => return None,
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
            self.expr_inner(current_precedence);
            lhs = marker.complete(self, BIN_EXPR);
        }

        Some(lhs)
    }

    // TODO: Move to utils
    pub(super) fn identifier(&mut self, wrapper: SyntaxKind) -> Option<CompletedMarker> {
        let marker = self.start();
        match self.current() {
            IDENT => self.bump(),

            _ => {
                let error = Report::build(
                    ReportKind::Error,
                    self.file,
                    self.current_span().start() as usize,
                )
                .with_message("Expected an identifier")
                .with_label(Label::new(self.current_span()).with_message("Expected an identifier"))
                .finish();
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
        while !self.at(T!['}']) {
            self.expr();
        }

        self.expect(T!['}']);
        let marker = block.complete(self, BLOCK);

        self.recovery_set = previous;
        Some(marker)
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
