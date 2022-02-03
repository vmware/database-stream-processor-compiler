use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{EXPR_STMT, VAR_DECL},
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
    // test(stmt) two_if_stmts
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
    pub(super) fn stmt(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();

        match self.peek() {
            T![let] => self.let_decl(),

            _ => {
                let expr_stmt = self.start();
                if self.expr().is_none() {
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
                    expr_stmt.abandon(self);
                    None

                // Otherwise, eat trailing semicolons
                } else {
                    self.eat_semicolons();
                    Some(expr_stmt.complete(self, EXPR_STMT))
                }
            }
        }
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
}
