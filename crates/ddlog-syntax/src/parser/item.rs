use crate::{
    constants::{FUNC_ARGS, FUNC_BODY, FUNC_DECL, FUNC_NAME},
    parser::{CompletedMarker, Marker, Parser},
};

// TODO: Error recovery
// TODO: Attributes

impl Parser<'_, '_> {
    pub(crate) fn items(&mut self) {
        while !self.at(T![eof]) {
            self.item();
        }
    }

    fn item(&mut self) {
        let marker = self.start();

        match self.peek() {
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![function] => {
                self.function_def(marker);
            }

            // TODO: Errors
            _ => marker.abandon(self),
        }
    }

    fn function_def(&mut self, marker: Marker) -> CompletedMarker {
        self.expect(T![function]);
        self.identifier(FUNC_NAME);

        let args = self.start();
        self.expect(T!['(']);
        // Params
        self.expect(T![')']);
        args.complete(self, FUNC_ARGS);

        if self.at(T![:]) {
            // return type
        }

        let body = self.start();
        self.expect(T!['{']);
        // Function body
        self.expr();
        self.expect(T!['}']);
        body.complete(self, FUNC_BODY);

        marker.complete(self, FUNC_DECL)
    }
}
