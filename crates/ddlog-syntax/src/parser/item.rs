use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{
        self, ATTRIBUTE, ATTRIBUTES, ATTR_PAIR, FUNCTION_ARG, FUNCTION_ARGS, FUNCTION_DEF,
        FUNCTION_NAME, FUNCTION_RETURN, FUNCTION_RETURN_TYPE, FUNCTION_TYPE, FUNCTION_TYPE_ARG,
        FUNCTION_TYPE_ARGS, GENERICS, GENERIC_ARG, GENERIC_TYPE, IDENT, ITEM, MODIFIERS,
        RECORD_FIELD, RECORD_NAME, RECORD_TYPE, RELATION_DEF, REL_COL, REL_COLS, REL_KW, REL_NAME,
        TUPLE_TYPE, TUPLE_TYPE_ELEM, TYPE, TYPE_BODY, TYPE_DEF, TYPE_NAME, VAR_REF,
    },
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

// TODO: Error recovery
// TODO: Attributes

const ITEM_RECOVERY: TokenSet = token_set! {
    T!['}'],
    T![function],
    T![extern],
    T![typedef],
    T![input],
    T![output],
    T![relation],
    T![multiset],
    T![stream],
    // T![import],
};

const MODIFIER_KEYWORDS: TokenSet = token_set! {
    T![input],
    T![output],
    T![extern],
};

impl Parser<'_, '_> {
    // test block_comments
    // - relation Foo()
    // - /*
    // -     /*
    // -         Hello
    // -     */
    // - */
    // - function bar() {}
    pub(crate) fn items(&mut self) {
        let current_set = self.recovery_set;
        self.recovery_set = current_set.union(ITEM_RECOVERY);

        while !self.at_end() {
            self.item();
        }

        self.recovery_set = current_set;
    }

    // TODO: Abandon attributes & modifiers if an error occurs in parsing the item
    fn item(&mut self) -> Option<CompletedMarker> {
        tracing::trace!(current = %self.current(), peek = %self.peek(), "parsing item");

        let item = self.start();
        let inner_item = self.start();

        self.attributes();
        // We eat any modifiers given to us, even though they aren't
        // all completely valid. For instance, this accepts `input extern input function`
        // even though that's 100% invalid, that's something that will be
        // caught during validation
        self.modifiers();

        tracing::trace!(peek = %self.peek(), "parsing item");
        match self.peek() {
            T![function] => self.function_def(inner_item),
            T![relation] | T![multiset] | T![stream] => self.relation_def(inner_item),
            T![typedef] => self.typedef(inner_item),

            // TODO: `import`

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

                inner_item.abandon(self);
                item.abandon(self);

                return None;
            }
        };

        Some(item.complete(self, ITEM))
    }

    // test function_definitions
    // - function foo() {}
    // - function foo1(bar: Baz) {}
    // - function foo2(bar: Baz, bing: Bang) {}
    fn function_def(&mut self, function: Marker) -> Option<CompletedMarker> {
        self.expect(T![function]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.identifier(FUNCTION_NAME);
        self.recovery_set = current_set;

        self.function_args();

        // test function_ret_ty
        // - function foo(): Bar {}
        if self.at(T![:]) {
            let ret = self.start();

            self.expect(T![:]);
            self.ty();

            ret.complete(self, FUNCTION_RETURN);
        }

        self.block();

        Some(function.complete(self, FUNCTION_DEF))
    }

    // test function_args
    // - function foo(bar: Baz, bing: Bang) {}
    fn function_args(&mut self) -> Option<CompletedMarker> {
        let args = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) && !self.at_end() {
            self.argument(FUNCTION_ARG);
        }

        self.expect(T![')']);

        Some(args.complete(self, FUNCTION_ARGS))
    }

    // test basic_relation
    // - relation Something()
    // test relation_with_multiple_modifiers
    // - input output relation Foo()
    // - input output multiset Foo()
    // - input output stream Foo()
    // test lowercase_relation
    // - relation foo()
    // test_err unclosed_relation_args
    // - relation Foo(
    // test streams_and_sets
    // - stream Bar()
    // - multiset Foo()
    fn relation_def(&mut self, relation: Marker) -> Option<CompletedMarker> {
        let keyword = self.start();
        if self.at(T![relation]) {
            self.expect(T![relation]);
        } else if self.at(T![multiset]) {
            self.expect(T![multiset]);
        } else if self.at(T![stream]) {
            self.expect(T![stream]);
        } else {
            unreachable!()
        }
        keyword.complete(self, REL_KW);

        self.identifier(REL_NAME);
        self.relation_columns();

        Some(relation.complete(self, RELATION_DEF))
    }

    // test relation_columns
    // - relation Foo(bar: Baz, bing: Bang)
    // - multiset Foo(bar: Baz, bing: Bang)
    // - stream Foo(bar: Baz, bing: Bang)
    fn relation_columns(&mut self) -> Option<CompletedMarker> {
        let columns = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) && !self.at_end() {
            self.argument(REL_COL);
        }

        self.expect(T![')']);

        Some(columns.complete(self, REL_COLS))
    }

    fn typedef(&mut self, typedef: Marker) -> Option<CompletedMarker> {
        self.expect(T![typedef]);
        self.identifier(TYPE_NAME);

        self.expect(T![=]);

        {
            let body = self.start();

            self.record_type();

            body.complete(self, TYPE_BODY);
        }

        Some(typedef.complete(self, TYPE_DEF))
    }

    // test typedefs
    // - typedef Foo = Bar
    // TODO: Generic type aliases
    fn record_type(&mut self) -> Option<CompletedMarker> {
        let record = self.start();

        self.identifier(RECORD_NAME);

        if self.try_expect(T!['{']) {
            while !self.at(T!['}']) {
                let field = self.start();

                self.pattern();
                self.expect(T![:]);
                self.ty();
                self.eat_commas();

                field.complete(self, RECORD_FIELD);
            }
            self.expect(T!['}']);
        }

        Some(record.complete(self, RECORD_TYPE))
    }

    // test argument_attributes
    // - function foo(
    // -     #[by_val]
    // -     bar: Baz,
    // - ) {}
    // - relation Bar(
    // -     #[by_val]
    // -     bar: Baz,
    // - )
    fn argument(&mut self, kind: SyntaxKind) -> Option<CompletedMarker> {
        let column = self.start();

        self.attributes();
        self.pattern();
        self.expect(T![:]);
        self.ty();
        self.eat_commas();

        Some(column.complete(self, kind))
    }

    // TODO: Extend to full types
    fn ty(&mut self) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let ty = self.start();

        match self.current() {
            IDENT => self.type_name(),
            T!['('] => self.tuple_type(),
            T![function] => self.function_type(),

            // FIXME: Error
            _ => {
                ty.abandon(self);
                return None;
            }
        };

        Some(ty.complete(self, TYPE))
    }

    // test generic_types
    // - function foo(bar: Bar<Baz>) {}
    // - relation Foo(bar: Bar<Baz>)
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
    // - function foo(bar: (Bar, Baz)) {}
    // - relation Foo(bar: (Bar, Baz,))
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
    // - function foo(bar: function(Bar, Baz): Bing) {}
    // - relation Foo(bar: function(Bar, Baz,): Bing,)
    // - function foo(bar: function(Bar)) {}
    // - relation Foo(bar: function(Bar,))
    fn function_type(&mut self) -> Option<CompletedMarker> {
        let function = self.start();

        self.expect(T![function]);

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

            // FIXME: Error recovery for missing `:`
            if self.try_expect(T![:]) {
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
    // - input input input input input output extern output input relation Foo()
    // - input input input input input output extern output input function foo() {}
    // - extern extern extern relation Foo()
    // - extern extern input extern stream Foo()
    // - extern extern extern function foo() {}
    fn modifiers(&mut self) -> CompletedMarker {
        let modifiers = self.start();
        while self.at_set(MODIFIER_KEYWORDS) {
            self.bump();
        }

        modifiers.complete(self, MODIFIERS)
    }

    // test attributes
    // - #[something = 10,,,]
    // - #[something_else = 10,,,]
    // - #[something_again = wheee,,,]
    // - function foobaz() {}
    fn attributes(&mut self) -> CompletedMarker {
        let attributes = self.start();

        while self.at(T!["#["]) {
            if self.attribute().is_none() {
                // TODO: Error handling & recovery
                break;
            }
        }

        attributes.complete(self, ATTRIBUTES)
    }

    // test attribute
    // - #[something = something_else] function foo() {}
    // - #[something = 10,,,] function foobar() {}
    // - #[by_val] function foobar() {}
    // - #[by_val, from_val] function foobar() {}
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
