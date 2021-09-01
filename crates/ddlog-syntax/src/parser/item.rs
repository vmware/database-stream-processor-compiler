use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{FUNC_ARGS, FUNC_DEF, FUNC_NAME, ITEM, RELATION_DEF, REL_MODS, REL_NAME},
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

// TODO: Error recovery
// TODO: Attributes

const ITEM_RECOVERY: TokenSet = token_set! {
    T!['}'],
    T![function],
    T![extern],
    // T![typedef],
    T![input],
    T![output],
    T![relation],
};

const MODIFIERS: TokenSet = token_set! {
    T![input],
    T![output],
    T![extern],
};

impl Parser<'_, '_> {
    pub(crate) fn items(&mut self) {
        let current_set = self.recovery_set;
        self.recovery_set = current_set.union(ITEM_RECOVERY);

        while !self.at_end() {
            self.item();
        }

        self.recovery_set = current_set;
    }

    fn item(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        match self.peek() {
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![function] => {
                self.function_def();
            }

            T![input] | T![output] | T![relation] => {
                self.relation_def();
            }

            // TODO: Errors
            _ => {
                let error_start = self.current_span();
                let error_end = self.error_eat_until(ITEM_RECOVERY);
                let error_span = error_start.merge(error_end);

                let error = Diagnostic::error()
                    .with_message("expected a top-level item")
                    .with_label(
                        Label::primary(error_span).with_message("expected a top-level item"),
                    );
                self.push_error(error);

                marker.abandon(self);
                return None;
            }
        }

        Some(marker.complete(self, ITEM))
    }

    // test function_definitions
    // function foo() {}
    fn function_def(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        self.expect(T![function]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.identifier(FUNC_NAME);
        self.recovery_set = current_set;

        let args = self.start();
        if !self.expect(T!['(']) {
            args.abandon(self);
            marker.complete(self, FUNC_DEF);

            return None;
        }

        // TODO: Params
        self.expect(T![')']);
        args.complete(self, FUNC_ARGS);

        if self.at(T![:]) {
            // return type
            self.expect(T![:]);
        }

        self.block(ITEM_RECOVERY);

        Some(marker.complete(self, FUNC_DEF))
    }

    fn eat_modifiers(&mut self) {
        while self.at_set(MODIFIERS) {
            self.bump();
        }
    }

    fn relation_def(&mut self) -> Option<CompletedMarker> {
        let relation = self.start();

        // We eat any modifiers given to us, even though they aren't
        // all completely valid. For instance, this accepts `input extern input relation`
        // even though that's 100% invalid, that's something that will be
        // caught during validation
        let modifiers = self.start();
        self.eat_modifiers();
        modifiers.complete(self, REL_MODS);

        self.expect(T![relation]);
        self.identifier(REL_NAME);

        self.expect(T!['(']);
        // TODO: Relation args
        self.expect(T![')']);

        Some(relation.complete(self, RELATION_DEF))
    }
}
