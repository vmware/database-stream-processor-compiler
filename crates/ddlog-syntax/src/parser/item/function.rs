use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{self, FUNCTION_ARG, FUNCTION_ARGS, FUNCTION_DEF, FUNCTION_RETURN},
};

impl Parser<'_, '_> {
    // test function_definitions
    // - fn foo() {}
    // - fn foo1(bar: Baz) {}
    // - fn foo2(bar: Baz, bing: Bang) {}
    pub(super) fn function_def(&mut self, function: Marker) -> Option<CompletedMarker> {
        self.expect(T![fn]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.ident();
        self.recovery_set = current_set;

        self.generics();
        self.function_args();

        // test function_ret_ty
        // - fn foo() -> Bar {}
        if self.at(T![->]) {
            let ret = self.start();

            self.expect(T![->]);
            self.ty();

            ret.complete(self, FUNCTION_RETURN);
        }

        // TODO: Detect unicode arrows like → and offer correction

        self.block();

        Some(function.complete(self, FUNCTION_DEF))
    }

    // test function_args
    // - fn foo(bar: Baz, bing: Bang) {}
    fn function_args(&mut self) -> Option<CompletedMarker> {
        let args = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) && !self.at_end() {
            self.argument(FUNCTION_ARG);
        }

        self.expect(T![')']);

        Some(args.complete(self, FUNCTION_ARGS))
    }

    // test argument_attributes
    // - fn foo(
    // -     #[by_val]
    // -     bar: Baz,
    // - ) {}
    fn argument(&mut self, kind: SyntaxKind) -> Option<CompletedMarker> {
        let column = self.start();

        self.attributes();
        self.pattern();
        self.expect(T![:]);
        self.ty();
        self.eat_commas();

        Some(column.complete(self, kind))
    }
}
