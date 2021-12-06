use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{BLOCK, ELSE_BLOCK, IF_BLOCK, IF_EXPR, RET_EXPR, VAR_DECL},
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

const STATEMENT_RECOVERY: TokenSet = token_set! {
    T![;],
    T![if],
    T![let],
    T![for],
    T!['{'],
    T!['}'],
    T![loop],
    T![match],
    T![while],
    // TODO: There's more of these
};

impl<'src, 'token> Parser<'src, 'token> {
    pub(super) fn stmt(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();

        match self.peek() {
            T![if] => self.parse_if(),
            T![let] => self.let_decl(),
            T!['{'] => self.block(true),
            T![return] => self.ret(true),

            _ => {
                let expr = self.expr();
                if expr.is_none() {
                    // TODO: Get full span text
                    let source = self.current_text();
                    let span = self.error_eat_until(STATEMENT_RECOVERY);

                    let error = Diagnostic::error()
                        .with_message("expected a statement")
                        .with_label(
                            Label::primary(span)
                                .with_message(format!("expected a statement, got '{}'", source)),
                        );

                    self.push_error(error);

                // Otherwise, eat trailing semicolons
                } else {
                    self.eat_semicolons();
                }

                expr
            }
        }
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

        Some(if_stmt.complete(self, IF_EXPR))
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
    // - let foo = bar;
    // - let baz = 1 + 100 + { 10 };
    fn let_decl(&mut self) -> Option<CompletedMarker> {
        let decl = self.start();

        self.expect(T![let]);
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
