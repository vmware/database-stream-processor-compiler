use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        FUNC_ARGS, FUNC_DEF, FUNC_MODS, FUNC_NAME, ITEM, RELATION_DEF, REL_MODS, REL_NAME,
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
        let marker = self.start();

        tracing::trace!(peek = %self.peek(), "parsing item");
        match self.peek() {
            // TODO: `extern`, `typedef`, `input`, `output`,
            //       `relation`, `apply`, etc.
            T![extern] | T![function] => {
                self.function_def();
            }

            T![input] | T![output] | T![relation] | T![multiset] | T![stream] => {
                self.relation_def();
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

                marker.abandon(self);
                return None;
            }
        }

        Some(marker.complete(self, ITEM))
    }

    // test function_definitions
    // - function foo() {}
    // test(validate) extern_function
    // - extern function foo() {}
    fn function_def(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        // We eat any modifiers given to us, even though they aren't
        // all completely valid. For instance, this accepts `input extern input function`
        // even though that's 100% invalid, that's something that will be
        // caught during validation
        // test_err(validate) double_extern_function
        // - extern extern function foo() {}
        // test_err(validate) input_function
        // - extern input function foo() {}
        // test_err(validate) output_function
        // - extern output function foo() {}
        let modifiers = self.start();
        self.eat_modifiers();
        modifiers.complete(self, FUNC_MODS);

        self.expect(T![function]);

        let current_set = self.recovery_set;
        self.recovery_set = current_set.add(T!['(']);
        self.identifier(FUNC_NAME);
        self.recovery_set = current_set;

        let args = self.start();
        if !self.expect(T!['(']) {
            args.abandon(self);
            marker.complete(self, FUNC_DEF);

            return None;
        }

        // TODO: Params
        self.expect(T![')']);
        args.complete(self, FUNC_ARGS);

        if self.at(T![:]) {
            // return type
            self.expect(T![:]);
        }

        self.block(ITEM_RECOVERY);

        Some(marker.complete(self, FUNC_DEF))
    }

    // test basic_relation
    // - relation Something()
    // test relation_with_multiple_modifiers
    // - input output relation Foo()
    // test lowercase_relation
    // - relation foo()
    // test_err unclosed_relation_args
    // - relation Foo(
    fn relation_def(&mut self) -> Option<CompletedMarker> {
        let relation = self.start();

        // We eat any modifiers given to us, even though they aren't
        // all completely valid. For instance, this accepts `input extern input relation`
        // even though that's 100% invalid, that's something that will be
        // caught during validation
        // test_err(validate) extern_relation
        // - input extern relation Foo()
        // - input extern extern relation Foo()
        // test_err(validate) relation_with_input_and_output
        // - input output relation Foo()
        let modifiers = self.start();
        self.eat_modifiers();
        modifiers.complete(self, REL_MODS);

        if self.at(T![relation]) {
            self.expect(T![relation]);
        } else if self.at(T![multiset]) {
            self.expect(T![multiset]);
        } else if self.at(T![stream]) {
            self.expect(T![stream]);
        } else {
            // TODO: Error
        }

        self.identifier(REL_NAME);

        self.expect(T!['(']);
        // TODO: Relation args
        self.expect(T![')']);

        Some(relation.complete(self, RELATION_DEF))
    }

    // test indiscriminantly_modify
    // - input input input input input output extern output input relation Foo()
    fn eat_modifiers(&mut self) {
        while self.at_set(MODIFIERS) {
            self.bump();
        }
    }
}
