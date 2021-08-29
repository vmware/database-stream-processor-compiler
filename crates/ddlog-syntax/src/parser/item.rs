use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{FUNC_ARGS, FUNC_DEF, FUNC_NAME, ITEM},
    TokenSet,
};
use ariadne::{Label, Report, ReportKind};

// TODO: Error recovery
// TODO: Attributes

const ITEM_RECOVERY_SET: TokenSet = token_set! {
    T!['}'],
    T![function],
    // T![extern],
    // T![typedef],
    // T![input],
    // T![output],
    // T![relation],
};

impl Parser<'_, '_> {
    pub(crate) fn items(&mut self) {
        while !self.at_end() {
            self.item();
        }
    }

    fn item(&mut self) {
        let marker = self.start();

        match self.peek() {
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![function] => {
                self.function_def();
            }

            // TODO: Errors
            _ => {
                let error = Report::build(
                    ReportKind::Error,
                    self.file,
                    self.current_span().start() as usize,
                )
                .with_message("expected a top-level item")
                .with_label(Label::new(self.current_span()).with_message("instead got this"))
                .finish();

                self.error(error);
            }
        }

        marker.complete(self, ITEM);
    }

    fn function_def(&mut self) -> CompletedMarker {
        let marker = self.start();

        self.expect(T![function]);
        self.identifier(FUNC_NAME);

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
