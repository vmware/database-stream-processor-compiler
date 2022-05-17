mod clause;
mod function;
mod relation;
mod structs;

use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{ATTRIBUTE, ATTR_PAIR, IDENT, MODIFIER},
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

// TODO: Error recovery
// TODO: Attributes

const ITEM_RECOVERY: TokenSet = token_set! {
    T!['}'],
    T![fn],
    T![pub],
    T![use],
    T![type],
    T![enum],
    T![impl],
    T![const],
    T![struct],
};

const MODIFIER_KEYWORDS: TokenSet = token_set! {
    T![pub],
};

impl Parser<'_, '_> {
    // test block_comments
    // - /*
    // -     /*
    // -         Hello
    // -     */
    // - */
    // - fn bar() {}
    pub(crate) fn items(&mut self) {
        let current_set = self.recovery_set;
        self.recovery_set = current_set.union(ITEM_RECOVERY);

        while !self.at_end() {
            if self.item().is_none() {
                break;
            }
        }

        self.recovery_set = current_set;
    }

    // TODO: Abandon attributes & modifiers if an error occurs in parsing the item
    // TODO: Attach comments to their parent item
    fn item(&mut self) -> Option<CompletedMarker> {
        tracing::trace!(current = %self.current(), peek = %self.peek(), "parsing item");
        let _frame = self.stack_frame();

        let item = self.start();

        self.attributes();
        // We eat any modifiers given to us, even though they aren't
        // all completely valid. For instance, this accepts `input extern input function`
        // even though that's 100% invalid, that's something that will be
        // caught during validation
        self.modifiers();

        match self.peek() {
            T![fn] => self.function_def(item),
            T![struct] => self.struct_def(item),
            T![rel] => self.relation_def(item),

            // Clauses are the only item that's started with identifiers
            IDENT => self.clause_def(item),

            // TODO: Enums
            // TODO: Constants
            // TODO: Impl blocks
            // TODO: Type aliases

            // TODO: Errors
            kind => {
                tracing::trace!("unexpected token for item '{}'", kind);

                let error_start = self.current_span();
                let error_end = self.error_eat_until(ITEM_RECOVERY);
                let error_span = error_start.merge(error_end);

                let error = Diagnostic::error()
                    .with_message("expected a top-level item")
                    .with_label(
                        Label::primary(error_span).with_message("expected a top-level item"),
                    );
                self.push_error(error);

                item.abandon(self);

                None
            }
        }
    }

    // test indiscriminantly_modify
    // - pub pub pub pub pub fn foo() {}
    fn modifiers(&mut self) {
        while self.at_set(MODIFIER_KEYWORDS) {
            if self.modifier().is_none() {
                break;
            }
        }
    }

    fn modifier(&mut self) -> Option<CompletedMarker> {
        let modifier = self.start();
        if self.at(T![pub]) {
            self.bump();
            Some(modifier.complete(self, MODIFIER))
        } else {
            tracing::trace!("unexpected token for modifier '{}'", self.current());

            let error_start = self.current_span();
            let error_end = self.error_eat_until(ITEM_RECOVERY);
            let error_span = error_start.merge(error_end);

            let error = Diagnostic::error()
                .with_message("expected a modifier")
                .with_label(
                    Label::primary(error_span).with_message("expected a modifier like `pub`"),
                );
            self.push_error(error);

            modifier.abandon(self);
            None
        }
    }

    // test attributes
    // - #[something = 10,,,]
    // - #[something_else = 10,,,]
    // - #[something_again = wheee,,,]
    // - fn foobaz() {}
    fn attributes(&mut self) {
        while self.at(T!["#["]) {
            if self.attribute().is_none() {
                // TODO: Error handling & recovery
                break;
            }
        }
    }

    // test attribute
    // - #[something = something_else] fn foo() {}
    // - #[something = 10,,,] fn foobar() {}
    // - #[by_val] fn foobar() {}
    // - #[by_val, from_val] fn foobar() {}
    fn attribute(&mut self) -> Option<CompletedMarker> {
        let attribute = self.start();

        self.expect(T!["#["]);
        while !self.at(T![']']) {
            let pair = self.start();

            self.expect(IDENT);
            // FIXME: Error recovery with missing `=` on a key-value pair
            if self.try_expect(T![=]) {
                self.expr();
            }

            self.eat_commas();

            pair.complete(self, ATTR_PAIR);
        }

        self.expect(T![']']);

        Some(attribute.complete(self, ATTRIBUTE))
    }
}
