use crate::{
    parser::{CompletedMarker, Marker, Parser},
    SyntaxKind::{
        self, FUNC_ARG, FUNC_ARGS, FUNC_DEF, FUNC_MODS, FUNC_NAME, IDENT, ITEM, RELATION_DEF,
        REL_COL, REL_COLS, REL_KW, REL_MODS, REL_NAME,
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

const MODIFIERS: TokenSet = token_set! {
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

        // We eat any modifiers given to us, even though they aren't
        // all completely valid. For instance, this accepts `input extern input function`
        // even though that's 100% invalid, that's something that will be
        // caught during validation
        let modifiers = self.start();
        self.eat_modifiers();
        // `modifiers` is completed by each individual item

        tracing::trace!(peek = %self.peek(), "parsing item");
        match self.peek() {
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![function] => {
                modifiers.complete(self, FUNC_MODS);
                self.function_def(inner_item);
            }

            T![relation] | T![multiset] | T![stream] => {
                modifiers.complete(self, REL_MODS);
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

                modifiers.abandon(self);
                inner_item.abandon(self);
                item.abandon(self);

                return None;
            }
        }

        Some(item.complete(self, ITEM))
    }

    // test function_definitions
    // - function foo() {}
    // test(validate) extern_function
    // - extern function foo() {}
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

            // FIXME: Eat multiple commas
            // test_err function_arg_without_comma
            // - function foo(bar: Baz bing: Bang) {}
            let at_comma = self.try_expect(T![,]);
            if !at_comma && !self.at(T![')']) {
                let error = Diagnostic::error()
                    .with_message("expected a comma between function arguments")
                    .with_label(
                        // FIXME: Fix this span
                        Label::primary(self.current_span()).with_message("expected a comma"),
                    );

                self.push_error(error);

            // test_err double_comma_function_arg
            // - function foo(bar: Baz,,) {}
            } else if at_comma && !self.at(T![')']) {
                if let Some(comma_span) = self.try_expect_span(T![,]) {
                    let error = Diagnostic::error()
                        .with_message("got multiple commas between function arguments")
                        .with_label(
                            Label::primary(comma_span).with_message("help: remove this comma"),
                        );

                    self.push_error(error);
                }
            }
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

            // FIXME: Eat multiple commas
            // test_err relation_arg_without_comma
            // - relation Foo(bar: Baz bing: Bang)
            let at_comma = self.try_expect(T![,]);
            if !at_comma && !self.at(T![')']) {
                let error = Diagnostic::error()
                    .with_message("expected a comma between relation arguments")
                    .with_label(
                        // FIXME: Fix this span
                        Label::primary(self.current_span()).with_message("expected a comma"),
                    );

                self.push_error(error);

            // test_err double_comma_relation_arg
            // - relation Foo(bar: Baz,,)
            } else if at_comma && !self.at(T![')']) {
                if let Some(comma_span) = self.try_expect_span(T![,]) {
                    let error = Diagnostic::error()
                        .with_message("got multiple commas between relation arguments")
                        .with_label(
                            Label::primary(comma_span).with_message("help: remove this comma"),
                        );

                    self.push_error(error);
                }
            }
        }

        self.expect(T![')']);

        Some(columns.complete(self, REL_COLS))
    }

    fn argument(&mut self, kind: SyntaxKind) -> Option<CompletedMarker> {
        let column = self.start();

        self.pattern();
        self.expect(T![:]);
        self.ty();

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
    // - input input input input input output extern output input function Foo()
    // - extern extern extern relation Foo()
    // - extern extern input extern stream Foo()
    // - extern extern extern function Foo()
    // test_err(validate) double_extern_function
    // - extern extern function foo() {}
    // test_err(validate) input_function
    // - extern input function foo() {}
    // test_err(validate) output_function
    // - extern output function foo() {}
    // test_err(validate) extern_relation
    // - input extern relation Foo()
    // - input extern extern relation Foo()
    // - input extern multiset Foo()
    // - input extern extern multiset Foo()
    // - input extern stream Foo()
    // - input extern extern stream Foo()
    // test_err(validate) relation_with_input_and_output
    // - input output relation Foo()
    fn eat_modifiers(&mut self) {
        while self.at_set(MODIFIERS) {
            self.bump();
        }
    }
}
