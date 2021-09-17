use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{BLOCK, ELSE_BLOCK, IF_BLOCK, IF_STMT, RET_EXPR, STMT, VAR_DECL},
};
use ddlog_diagnostics::{Diagnostic, Label};

impl<'src, 'token> Parser<'src, 'token> {
    pub(super) fn stmt(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let stmt = self.start();

        match self.peek() {
            T![if] => {
                self.parse_if();
            }
            T![var] => {
                self.var_decl();
            }
            T!['{'] => {
                self.block(true);
            }
            T![return] => {
                self.ret(true);
            }

            _ => {
                if self.expr().is_none() {
                    // TODO: Get full span text
                    let source = self.current_text();
                    let span = self
                        .error_eat_until(token_set! { T!['{'], T!['}'], T![;], T![if], T![var] });

                    let error = Diagnostic::error()
                        .with_message("expected a statement")
                        .with_label(
                            Label::primary(span)
                                .with_message(format!("expected a statement, got '{}'", source)),
                        );

                    self.push_error(error);

                    stmt.abandon(self);
                    return None;

                // Otherwise, eat trailing semicolons
                } else {
                    self.eat_semicolons();
                }
            }
        }

        Some(stmt.complete(self, STMT))
    }

    // test(stmt) if_stmts
    // - if x {
    // -     x + y;
    // - } else if y {
    // -     x + y;
    // - } else if z {
    // -     x + y;
    // - } else {
    // -     x + y;
    // - }
    // test(stmt) two_ifs
    // - if x {
    // -     x + y;
    // - }
    // - if y {
    // -     x + y;
    // - } else if z {
    // -     x + y;
    // - } else {
    // -     x + y;
    // - }
    pub(super) fn parse_if(&mut self) -> Option<CompletedMarker> {
        let if_stmt = self.start();

        self.if_block(None);
        while self.at(T![else]) {
            let block = self.start();
            self.expect(T![else]);

            if self.at(T![if]) {
                self.if_block(Some(block));
            } else {
                self.else_block(Some(block));
            }
        }

        if self.at(T![else]) {
            let block = self.start();
            self.expect(T![else]);

            self.else_block(Some(block));
        }

        Some(if_stmt.complete(self, IF_STMT))
    }

    fn if_block(&mut self, block: Option<Marker>) -> Option<CompletedMarker> {
        let block = block.unwrap_or_else(|| self.start());

        self.expect(T![if]);
        self.expr();
        self.block(false);

        Some(block.complete(self, IF_BLOCK))
    }

    fn else_block(&mut self, block: Option<Marker>) -> Option<CompletedMarker> {
        let block = block.unwrap_or_else(|| self.start());

        self.block(false);

        Some(block.complete(self, ELSE_BLOCK))
    }

    // test(stmt) variable_declarations
    // - var foo = bar;
    // - var baz = 1 + 100 + { 10 };
    fn var_decl(&mut self) -> Option<CompletedMarker> {
        let decl = self.start();

        self.expect(T![var]);
        self.pattern();
        self.expect(T![=]);
        self.expr();
        self.eat_semicolons();

        Some(decl.complete(self, VAR_DECL))
    }

    // test(expr) block_exprs
    // - { 10 } - {{ 5 + ({ 99 }) }}
    // test_err(expr) unclosed_block
    // - {{ 10 }
    pub(super) fn block(&mut self, semicolons: bool) -> Option<CompletedMarker> {
        let block = self.start();
        if !self.expect(T!['{']) {
            block.abandon(self);
            return None;
        }

        // FIXME: Statements, not expressions
        let mut did_error = false;
        while !self.at(T!['}']) && !self.at_end() {
            if self.stmt().is_none() {
                // Bump so we don't loop forever
                self.error_eat_until(token_set! { T!['}'], T![;] });
                did_error = true;

                break;
            }
        }

        self.expect(T!['}']);
        if semicolons {
            self.eat_semicolons();
        }

        let marker = block.complete(self, BLOCK);

        if did_error {
            None
        } else {
            Some(marker)
        }
    }

    pub(super) fn ret(&mut self, semicolons: bool) -> Option<CompletedMarker> {
        let ret = self.start();

        self.expect(T![return]);
        self.expr();

        if semicolons {
            self.eat_semicolons();
        }

        Some(ret.complete(self, RET_EXPR))
    }
}
