use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{GENERICS, GENERIC_ARG, PATH, PATH_TAIL},
};

impl Parser<'_, '_> {
    // test generic_functions
    // - fn foo<>() {}
    // - fn foo1<A>() {}
    // - fn foo2<A, B, C, D, E>() {}
    // - fn foo3<A, B, C, D, E,>() {}
    pub(super) fn generics(&mut self) -> Option<CompletedMarker> {
        if self.try_expect(T![<]) {
            let generics = self.start();
            while !self.try_expect(T![>]) {
                let arg = self.start();
                self.ty().unwrap();
                self.eat_commas();
                arg.complete(self, GENERIC_ARG);
            }

            Some(generics.complete(self, GENERICS))
        } else {
            None
        }
    }

    // TODO: Tests
    pub(super) fn path(&mut self) -> CompletedMarker {
        let path = self.start();
        self.eat(T![::]);
        self.ident();

        while self.at(T![::]) {
            let tail = self.start();
            self.expect(T![::]);
            self.ident();
            tail.complete(self, PATH_TAIL);
        }

        path.complete(self, PATH)
    }
}
