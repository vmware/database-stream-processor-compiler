use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{RELATION_ARG, RELATION_ARGS, RELATION_DEF},
};

impl Parser<'_, '_> {
    // test relations
    // - rel Foo()
    // - rel Bar(x: usize)
    // - rel Baz(x: usize, y: Whee, z: Whoo<Bing, fn(usize) -> bool>)
    pub(super) fn relation_def(&mut self, relation: Marker) -> Option<CompletedMarker> {
        self.expect(T![rel]);
        self.ident();

        // Parse all arguments of the relation
        {
            let args = self.start();
            self.expect(T!['(']);

            // Parse each argument
            while !self.at(T![')']) {
                let arg = self.start();
                self.attributes();
                self.ident();
                self.expect(T![:]);
                self.ty().unwrap();
                self.eat_commas();
                arg.complete(self, RELATION_ARG);
            }

            self.expect(T![')']);
            args.complete(self, RELATION_ARGS);
        }

        Some(relation.complete(self, RELATION_DEF))
    }
}
