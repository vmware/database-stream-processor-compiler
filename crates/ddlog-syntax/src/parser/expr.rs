use ariadne::{Label, Report, ReportKind};

use crate::{
    parser::{CompletedMarker, Parser},
    syntax_kind::constants::{BINOP, LITERAL, NAME_REF, PARENS, PREFIX},
    SyntaxKind,
};

impl Parser<'_, '_> {
    pub(super) fn expr(&mut self) {
        self.expr_inner(0);
    }

    /// The innards of [`Parser::expr()`], does all of the actual work
    ///
    /// Utilizes [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    /// to parse operator precedence
    fn expr_inner(&mut self, min_precedence: u8) {
        let mut lhs = match self.peek() {
            // Identifiers are atomic
            T![ident] => {
                let marker = self.start();
                self.bump();

                marker.complete(self, NAME_REF)
            }

            // Numbers are atomic
            T![number] => {
                let marker = self.start();
                self.bump();

                marker.complete(self, LITERAL)
            }

            // Negation
            T![-] => {
                let op = Prefix::Neg;
                let ((), right_precedence) = op.precedence();
                let marker = self.start();

                // Eat the operator’s token.
                self.bump();
                self.expr_inner(right_precedence);

                marker.complete(self, PREFIX)
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

                marker.complete(self, PARENS)
            }

            // TODO: Error handling
            _ => return,
        };

        // Infix operators
        while !self.at(T![eof]) {
            let op = match self.peek() {
                T![+] => Infix::Add,
                T![-] => Infix::Sub,
                T![*] => Infix::Mul,
                T![/] => Infix::Div,

                // TODO: Error handling
                _ => return,
            };

            let (left_precedence, right_precedence) = op.precedence();
            if left_precedence < min_precedence {
                break;
            }

            // Eat the operator’s token.
            self.bump();

            let marker = lhs.precede(self);
            self.expr_inner(right_precedence);
            lhs = marker.complete(self, BINOP);
        }
    }

    // TODO: Move to utils
    pub(super) fn identifier(&mut self, wrapper: SyntaxKind) -> Option<CompletedMarker> {
        let marker = self.start();
        match self.current() {
            T![ident] => self.bump(),

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
}

/// A prefix expression
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Prefix {
    Neg,
}

impl Prefix {
    fn precedence(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

/// An infix expression
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Infix {
    Add,
    Sub,
    Mul,
    Div,
}

impl Infix {
    fn precedence(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}
