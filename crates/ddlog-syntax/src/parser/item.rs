use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{
        self, ATTRIBUTE, ATTRIBUTES, ATTR_PAIR, FUNC_ARG, FUNC_ARGS, FUNC_DEF, FUNC_NAME, IDENT,
        ITEM, MODIFIERS, RELATION_DEF, REL_COL, REL_COLS, REL_KW, REL_NAME,
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
    // T![typedef],
    T![input],
    T![output],
    T![relation],
    T![multiset],
    T![stream],
};

const MODIFIER_KEYWORDS: TokenSet = token_set! {
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
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![function] => {
                self.function_def(inner_item);
            }

            T![relation] | T![multiset] | T![stream] => {
                self.relation_def(inner_item);
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

                inner_item.abandon(self);
                item.abandon(self);

                return None;
            }
        }

        Some(item.complete(self, ITEM))
    }

    // test function_definitions
    // - function foo() {}
    fn function_def(&mut self, function: Marker) -> Option<CompletedMarker> {
        self.expect(T![function]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.identifier(FUNC_NAME);
        self.recovery_set = current_set;

        self.function_args();

        // test function_ret_ty
        // - function foo(): Bar {}
        if self.at(T![:]) {
            self.expect(T![:]);
            self.ty();
        }

        self.block(ITEM_RECOVERY);

        Some(function.complete(self, FUNC_DEF))
    }

    // test function_args
    // - function foo(bar: Baz, bing: Bang) {}
    fn function_args(&mut self) -> Option<CompletedMarker> {
        let args = self.start();
        self.expect(T!['(']);

        while !self.at(T![')']) && !self.at_end() {
            self.argument(FUNC_ARG);
        }

        self.expect(T![')']);

        Some(args.complete(self, FUNC_ARGS))
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

    fn argument(&mut self, kind: SyntaxKind) -> Option<CompletedMarker> {
        let column = self.start();

        self.pattern();
        self.expect(T![:]);
        self.ty();

        while self.try_expect(T![,]) {}

        Some(column.complete(self, kind))
    }

    // TODO: Extend to full types
    fn ty(&mut self) -> bool {
        self.expect(IDENT)
    }

    // TODO: Extend to full patterns
    fn pattern(&mut self) -> bool {
        self.expect(IDENT)
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

    // test attributes
    // - #[something = something_else] function foo() {}
    // - #[something = 10,,,] function foo() {}
    fn attribute(&mut self) -> Option<CompletedMarker> {
        let attribute = self.start();

        self.expect(T!["#["]);
        while !self.at(T![']']) {
            let pair = self.start();

            self.expect(IDENT);
            self.expect(T![=]);
            self.expr();

            while self.try_expect(T![,]) {}

            pair.complete(self, ATTR_PAIR);
        }

        self.expect(T![']']);

        Some(attribute.complete(self, ATTRIBUTE))
    }
}

// test block_comments
// - /*
// -     /*
// -         Hello
// -     */
// - */
