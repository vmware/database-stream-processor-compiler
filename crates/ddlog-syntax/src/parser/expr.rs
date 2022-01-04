use crate::{
    parser::{CompletedMarker, Parser},
    SyntaxKind::{
        self, BIN_EXPR, BIN_OP, BOOL, CHAR, CHAR_LITERAL, CLOSURE_ARG, CLOSURE_EXPR, FIELD_ACCESS,
        FIELD_ACCESSOR, FIELD_ACCESSOR_NAME, IDENT, METHOD_CALL, METHOD_CALL_ARG, NUMBER,
        NUMBER_LITERAL, PAREN_EXPR, STRING, STRING_LITERAL, TUPLE_EXPR_ELEM, TUPLE_INIT_EXPR,
        UNARY_EXPR, UNARY_OP, VAR_REF,
    },
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};

pub(super) const EXPR_RECOVERY_SET: TokenSet = token_set! {
    T![')'],
    T!['}'],
    T![;],
};

impl Parser<'_, '_> {
    // test(stmt) binary_ops
    // - {
    // -     1 == 2;
    // -     foo != bar;
    // -     1 * 2;
    // -     1 * 3 != 3 * 1;
    // -     1 == 2 and 5 == 10;
    // -     1 == 2 or 5 == 10;
    // -     (1 == 2 or 5 == 10) and (1 == 2 and 5 == 10);
    // -     1 + 2 + 3 + 4;
    // -     1 + 2 * 3 - 4;
    // - }
    //
    // test(expr) infix_negation
    // - -10
    //
    // test(expr) negation_has_higher_binding_power_than_infix
    // - -20 + 20
    //
    // test(expr) parentheses_affect_precedence
    // - 5 * (2 + 1)
    //
    // test(expr) method_call
    // - foo.bar()
    //
    // test(expr) field_access
    // - foo.bar
    //
    // test(expr) chained_field_access
    // - foo.bar.baz.bing
    pub(super) fn expr(&mut self) -> Option<CompletedMarker> {
        self.expr_inner(0)
    }

    /// The innards of [`Parser::expr()`], does all of the actual work
    ///
    /// Utilizes [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    /// to parse operator precedence
    // FIXME: Make this iterative instead of recursive
    // FIXME: This needs to be totally restructured, this fundamentally
    //        hinders using pratt parsing
    // FIXME: Function calls
    fn expr_inner(&mut self, mut current_precedence: u8) -> Option<CompletedMarker> {
        let _frame = self.stack_frame();
        let mut lhs = match self.peek() {
            IDENT => self.var_ref()?,
            NUMBER_LITERAL | STRING_LITERAL | CHAR_LITERAL | T![true] | T![false] => {
                self.literal()?
            }
            operator @ (T![-] | T![!]) => self.unary_expr(operator)?,
            T!['('] => self.parens_or_tuple()?,
            T!['{'] => self.block(false)?,
            T![return] => self.ret(false)?,
            T![|] => self.closure()?,

            _ => return None,
        };

        // Infix operators
        while !self.at_end() {
            let precedence = match infix_precedence(self.peek()) {
                Some(operand) => operand,
                // TODO: Error handling
                None => break,
            };

            // Eat the operator’s token.
            let inner = self.start();
            match self.peek() {
                // Binary operation
                T![or]
                | T![and]
                | T![==]
                | T![!=]
                | T![<]
                | T![<=]
                | T![>]
                | T![>=]
                | T![|]
                | T![^]
                | T![&]
                | T![<<]
                | T![>>]
                | T![+]
                | T![-]
                | T![*]
                | T![/]
                | T![%] => {
                    self.bump();
                    inner.complete(self, BIN_OP);

                    let marker = lhs.precede(self);
                    let rhs_invalid = self.expr_inner(precedence).is_none();
                    lhs = marker.complete(self, BIN_EXPR);

                    if precedence < current_precedence {
                        break;
                    }
                    current_precedence = precedence;

                    if rhs_invalid {
                        break;
                    }
                }

                // Method call
                T![.] => {
                    self.bump();
                    inner.abandon(self);

                    let marker = lhs.precede(self);

                    // Tuple access
                    lhs = if self.at(NUMBER_LITERAL) {
                        let accessor = self.start();
                        self.bump();
                        accessor.complete(self, NUMBER);

                        marker.complete(self, FIELD_ACCESSOR)

                    // Field access or method calls
                    } else {
                        let accessor = self.start();
                        let accessor_inner = self.start();
                        self.ident();

                        // Method call
                        if self.try_expect(T!['(']) {
                            accessor.abandon(self);
                            accessor_inner.abandon(self);

                            while !self.at(T![')']) {
                                let arg = self.start();

                                // TODO: Error handling
                                self.expr().unwrap();
                                self.eat_commas();

                                arg.complete(self, METHOD_CALL_ARG);
                            }

                            // TODO: Method args
                            self.expect(T![')']);
                            marker.complete(self, METHOD_CALL)

                        // Field access
                        } else {
                            accessor_inner.complete(self, FIELD_ACCESSOR_NAME);
                            accessor.complete(self, FIELD_ACCESSOR);
                            marker.complete(self, FIELD_ACCESS)
                        }
                    };

                    if precedence < current_precedence {
                        break;
                    }
                    current_precedence = precedence;
                }

                token => unimplemented!("unknown infix expression: {}", token),
            }
        }

        // TODO: Postfix expressions, e.g. `array[x]`

        Some(lhs)
    }

    // test(expr) parenthesised
    // - (((((((((2 + (((((5))))))))))))))
    // test_err(expr) unclosed_paren
    // - (((100))
    // test(expr) parenthesised_literal
    // - (1)
    // test(expr) single_element_tuple
    // - (1,)
    // test(expr) tuple_literals
    // - (1,)
    // - (1, 2)
    // - (1, 2,)
    // - (1, 2, 3)
    // - (1, 2, 3,)
    // test_err(expr) missing_tuple_comma
    // - (1, 2 3)
    fn parens_or_tuple(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        if !self.expect(T!['(']) {
            marker.abandon(self);
            return None;
        }

        let first_expr = self.start();
        self.expr();

        let kind = if self.try_expect(T![,]) {
            first_expr.complete(self, TUPLE_EXPR_ELEM);

            while !self.at(T![')']) {
                let tuple_elem = self.start();

                self.expr();
                self.try_expect(T![,]);

                tuple_elem.complete(self, TUPLE_EXPR_ELEM);
            }

            TUPLE_INIT_EXPR
        } else {
            first_expr.abandon(self);
            PAREN_EXPR
        };

        self.expect(T![')']);

        Some(marker.complete(self, kind))
    }

    // test(expr) negation
    // - --(-100)
    // test(expr) boolean_not
    // - !!(!true)
    fn unary_expr(&mut self, operator: SyntaxKind) -> Option<CompletedMarker> {
        let precedence = match unary_precedence(operator) {
            Some(operand) => operand,
            None => unreachable!(),
        };

        let marker = self.start();

        // Eat the operator’s token.
        let operand = self.start();
        self.expect(operator);
        operand.complete(self, UNARY_OP);

        let expr_invalid = self.expr_inner(precedence).is_none();

        let complete = marker.complete(self, UNARY_EXPR);
        if expr_invalid {
            return None;
        }

        Some(complete)
    }

    // test(expr) decimal_number
    // - 123
    // test(expr) hex_number
    // - 0xFFFF
    // test(expr) binary_number
    // - 0b010101
    // test(expr) bool_true
    // - true
    // test(expr) bool_false
    // - false
    // test(expr) strings
    // - "foo"
    // - "bar\"\n"
    // test(expr) chars
    // - 'c'
    pub(super) fn literal(&mut self) -> Option<CompletedMarker> {
        let literal = self.start();

        // Numbers
        if self.at(NUMBER_LITERAL) {
            self.bump();
            Some(literal.complete(self, NUMBER))

        // Strings
        } else if self.at(STRING_LITERAL) {
            self.bump();
            Some(literal.complete(self, STRING))

        // Booleans
        } else if self.at(T![true]) || self.at(T![false]) {
            self.bump();
            Some(literal.complete(self, BOOL))

        // Chars
        } else if self.at(CHAR_LITERAL) {
            self.bump();
            Some(literal.complete(self, CHAR))

        // Non-literals
        } else {
            literal.abandon(self);
            None
        }
    }

    // test(expr) var_ref
    // - a
    // - abcd
    // - abcd1245
    // - _
    // - _135
    // - _dsg434
    fn var_ref(&mut self) -> Option<CompletedMarker> {
        let var = self.start();
        self.ident();
        Some(var.complete(self, VAR_REF))
    }

    // TODO: Move to utils
    // test(expr) var_refs
    // - foo1245__
    // test(expr) underline_var
    // - _
    pub(super) fn ident(&mut self) {
        if self.at(IDENT) {
            self.bump();
        } else {
            let error = Diagnostic::error()
                .with_message("expected an identifier")
                .with_label(
                    Label::primary(self.current_span()).with_message("expected an identifier"),
                );
            self.error(error);
        }
    }

    // test(expr) toilet
    // - || {}
    //
    // test(expr) identity
    // - |x| x
    //
    // test(expr) block_closure
    // - |foo, bar| {
    // -     return baz(foo + bar);
    // - }
    fn closure(&mut self) -> Option<CompletedMarker> {
        let closure = self.start();

        self.expect(T![|]);
        while !self.at(T![|]) {
            let arg = self.start();

            // TODO: Error handling
            self.pattern().unwrap();

            if self.try_expect(T![:]) {
                // TODO: Error handling
                self.ty().unwrap();
            }

            self.eat_commas();
            arg.complete(self, CLOSURE_ARG);
        }
        self.expect(T![|]);

        // TODO: Error handling
        self.expr().unwrap();

        Some(closure.complete(self, CLOSURE_EXPR))
    }
}

fn unary_precedence(operand: SyntaxKind) -> Option<u8> {
    let precedence = match operand {
        T![-] | T![!] => 1,
        _ => return None,
    };

    Some(precedence)
}

fn infix_precedence(operand: SyntaxKind) -> Option<u8> {
    let precedence = match operand {
        T![or] => 1,
        T![and] => 2,
        T![==] | T![!=] | T![<] | T![<=] | T![>] | T![>=] => 3,
        T![|] => 4,
        T![^] => 5,
        T![&] => 6,
        T![<<] | T![>>] => 7,
        T![+] | T![-] => 8,
        T![*] | T![/] | T![%] => 9,
        T![.] => 10,

        _ => return None,
    };

    Some(precedence)
}
