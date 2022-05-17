use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{
        BRACKETED_STRUCT_FIELD, BRACKETED_STRUCT_FIELDS, STRUCT_DEF, TUPLE_STRUCT_FIELD,
        TUPLE_STRUCT_FIELDS,
    },
};

impl Parser<'_, '_> {
    // test struct_defs
    // - struct Foo {}
    // - struct Foo1 {
    // -     bar: usize,
    // - }
    // - struct Foo2 {
    // -     bar: usize
    // - }
    // - struct TupleStruct();
    // - struct TupleStruct(u8, u8, u8);
    // - struct Foo<T>(T);
    pub(super) fn struct_def(&mut self, struct_def: Marker) -> Option<CompletedMarker> {
        self.expect(T![struct]);
        self.ident();
        self.generics();

        if self.at(T!['{']) {
            self.bracketed_struct_fields();
        } else if self.at(T!['(']) {
            self.tuple_struct_fields();
        } else {
            // TODO: Error handling
        }

        Some(struct_def.complete(self, STRUCT_DEF))
    }

    fn bracketed_struct_fields(&mut self) -> Option<CompletedMarker> {
        let fields = self.start();
        self.expect(T!['{']);

        while !self.at(T!['}']) {
            // TODO: Error handling
            self.bracketed_struct_field();
        }

        self.expect(T!['}']);
        Some(fields.complete(self, BRACKETED_STRUCT_FIELDS))
    }

    fn bracketed_struct_field(&mut self) -> Option<CompletedMarker> {
        let field = self.start();

        // TODO: Error handling
        self.ident();
        self.eat(T![:]);
        self.ty();
        self.eat_commas();

        Some(field.complete(self, BRACKETED_STRUCT_FIELD))
    }

    fn tuple_struct_fields(&mut self) -> Option<CompletedMarker> {
        let fields = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) {
            // TODO: Error handling
            self.tuple_struct_field();
        }

        self.expect(T![')']);
        self.eat_semicolons();

        Some(fields.complete(self, TUPLE_STRUCT_FIELDS))
    }

    fn tuple_struct_field(&mut self) -> Option<CompletedMarker> {
        let field = self.start();

        // TODO: Error handling
        self.ty();
        self.eat_commas();

        Some(field.complete(self, TUPLE_STRUCT_FIELD))
    }
}
