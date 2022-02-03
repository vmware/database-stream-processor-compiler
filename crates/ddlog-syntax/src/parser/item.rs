use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{
        self, ATTRIBUTE, ATTR_PAIR, BRACKETED_STRUCT_FIELD, BRACKETED_STRUCT_FIELDS, FUNCTION_ARG,
        FUNCTION_ARGS, FUNCTION_DEF, FUNCTION_RETURN, FUNCTION_RETURN_TYPE, FUNCTION_TYPE,
        FUNCTION_TYPE_ARG, FUNCTION_TYPE_ARGS, GENERICS, GENERIC_ARG, GENERIC_TYPE, IDENT,
        MODIFIER, STRUCT_DEF, STRUCT_FIELDS, TUPLE_STRUCT_FIELD, TUPLE_STRUCT_FIELDS, TUPLE_TYPE,
        TUPLE_TYPE_ELEM, TYPE, VAR_REF,
    },
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

    // test function_definitions
    // - fn foo() {}
    // - fn foo1(bar: Baz) {}
    // - fn foo2(bar: Baz, bing: Bang) {}
    fn function_def(&mut self, function: Marker) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        self.expect(T![fn]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.ident();
        self.recovery_set = current_set;

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
    fn struct_def(&mut self, struct_def: Marker) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();

        self.expect(T![struct]);
        self.ident();

        let fields = self.start();
        if self.at(T!['{']) {
            self.bracketed_struct_fields();
        } else if self.at(T!['(']) {
            self.tuple_struct_fields();
        } else {
            // TODO: Error handling
        }
        fields.complete(self, STRUCT_FIELDS);

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

    // TODO: Extend to full types
    pub(super) fn ty(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let ty = self.start();

        match self.current() {
            IDENT => self.type_name(),
            T!['('] => self.tuple_type(),
            T![fn] => self.function_type(),

            // FIXME: Error
            _ => {
                ty.abandon(self);
                return None;
            }
        };

        Some(ty.complete(self, TYPE))
    }

    // test generic_types
    // - fn foo(bar: Bar<Baz>) {}
    fn type_name(&mut self) -> Option<CompletedMarker> {
        let generic = self.start();

        if self.expect(IDENT) && self.at(T![<]) {
            let generics = self.start();

            self.expect(T![<]);
            while !self.at(T![>]) {
                let generic_arg = self.start();

                self.ty();
                self.eat_commas();

                generic_arg.complete(self, GENERIC_ARG);
            }

            self.expect(T![>]);
            generics.complete(self, GENERICS);
        }

        Some(generic.complete(self, GENERIC_TYPE))
    }

    // test tuple_types
    // - fn foo(bar: (Bar, Baz)) {}
    fn tuple_type(&mut self) -> Option<CompletedMarker> {
        let tuple = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) {
            let elem = self.start();

            self.ty();
            self.eat_commas();

            elem.complete(self, TUPLE_TYPE_ELEM);
        }

        self.expect(T![')']);
        Some(tuple.complete(self, TUPLE_TYPE))
    }

    // test function_types
    // - fn foo(bar: fn(Bar, Baz) -> Bing) {}
    // - fn Foo(bar: fn(Bar, Baz,) -> Bing,) {}
    // - fn foo(bar: fn(Bar)) {}
    // - fn Foo(bar: fn(Bar,)) -> fn(Bong, Bang) {}
    fn function_type(&mut self) -> Option<CompletedMarker> {
        let function = self.start();

        self.expect(T![fn]);

        {
            let args = self.start();
            self.expect(T!['(']);

            while !self.at(T![')']) {
                let arg = self.start();

                self.ty();
                self.eat_commas();

                arg.complete(self, FUNCTION_TYPE_ARG);
            }

            self.expect(T![')']);
            args.complete(self, FUNCTION_TYPE_ARGS);
        }

        {
            let ret = self.start();

            // FIXME: Error recovery for missing `->`
            if self.try_expect(T![->]) {
                self.ty();
            }

            ret.complete(self, FUNCTION_RETURN_TYPE);
        }

        Some(function.complete(self, FUNCTION_TYPE))
    }

    // TODO: Extend to full patterns
    pub(super) fn pattern(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let pattern = self.start();

        self.expect(IDENT);

        Some(pattern.complete(self, VAR_REF))
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
