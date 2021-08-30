use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{FUNC_ARGS, FUNC_DEF, FUNC_NAME, ITEM},
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

// TODO: Error recovery
// TODO: Attributes

const ITEM_RECOVERY_SET: TokenSet = token_set! {
    T!['}'],
    T![function],
    // T![extern],
    // T![typedef],
    T![input],
    T![output],
    T![relation],
};

impl Parser<'_, '_> {
    pub(crate) fn items(&mut self) {
        while !self.at_end() {
            self.item();
        }
    }

    #[must_use]
    fn item(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        match self.peek() {
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![function] => {
                self.function_def();
            }

            // TODO: Errors
            _ => {
                let error = Diagnostic::error()
                    .with_message("expected a top-level item")
                    .with_label(
                        Label::primary(self.current_span())
                            .with_message("expected a top-level item"),
                    );

                self.error(error);

                marker.abandon(self);
                return None;
            }
        }

        Some(marker.complete(self, ITEM))
    }

    fn function_def(&mut self) -> CompletedMarker {
        let marker = self.start();

        self.expect(T![function]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.identifier(FUNC_NAME);
        self.recovery_set = current_set;

        let args = self.start();
        self.expect(T!['(']);
        // Params
        self.expect(T![')']);
        args.complete(self, FUNC_ARGS);

        if self.at(T![:]) {
            // return type
            self.expect(T![:]);
        }

        self.block(ITEM_RECOVERY_SET);

        marker.complete(self, FUNC_DEF)
    }
}
