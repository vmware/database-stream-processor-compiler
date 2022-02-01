use crate::{
    parser::{precedence::ExprPrecedence, CompletedMarker, Parser},
    SyntaxKind::{
        self, ASSIGN, ASSIGN_OP, BIN_EXPR, BIN_OP, BOOL, BREAK_EXPR, CHAR, CHAR_LITERAL,
        CLOSURE_ARG, CLOSURE_EXPR, CONTINUE_EXPR, FIELD_ACCESS, FIELD_ACCESSOR,
        FIELD_ACCESSOR_NAME, FUNCTION_CALL, FUNCTION_CALL_ARG, IDENT, METHOD_CALL, METHOD_CALL_ARG,
        NUMBER, NUMBER_LITERAL, PAREN_EXPR, RET_EXPR, STRING, STRING_LITERAL, TUPLE_EXPR_ELEM,
        TUPLE_INIT_EXPR, UNARY_EXPR, UNARY_OP, VAR_REF,
    },
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};
use std::convert::TryFrom;

type PrefixParselet<'src, 'token> =
    fn(&mut Parser<'src, 'token>, SyntaxKind) -> Option<CompletedMarker>;
type PostfixParselet<'src, 'token> =
    fn(&mut Parser<'src, 'token>, SyntaxKind, CompletedMarker) -> Option<CompletedMarker>;
type InfixParselet<'src, 'token> =
    fn(&mut Parser<'src, 'token>, SyntaxKind, CompletedMarker) -> Option<CompletedMarker>;

pub(super) const EXPR_RECOVERY_SET: TokenSet = token_set! {
    T![')'],
    T!['}'],
    T![;],
};

impl<'src, 'token> Parser<'src, 'token> {
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
    pub(super) fn expr_inner(&mut self, precedence: u8) -> Option<CompletedMarker> {
        let mut token = self.peek();

        let prefix = Self::expr_prefix(token);
        if let Some(prefix) = prefix {
            let mut left = prefix(self, token)?;

            if let Some(peek) = self.try_peek() {
                let postfix = Self::expr_postfix(peek);

                if let Some(postfix) = postfix {
                    token = self.peek();
                    left = postfix(self, token, left)?;
                }
            }

            while precedence < self.expr_precedence() {
                token = self.peek();

                let infix = Self::expr_infix(token);
                if let Some(infix) = infix {
                    left = infix(self, token, left)?;
                } else {
                    break;
                }
            }

            Some(left)
        } else {
            // TODO: Emit an error
            // Err(Locatable::new(
            //     Error::Syntax(SyntaxError::Generic(format!(
            //         "Could not parse `{}`",
            //         token.ty()
            //     ))),
            //     Location::new(&token, self.current_file),
            // ))
            None
        }
    }

    fn expr_prefix(kind: SyntaxKind) -> Option<PrefixParselet<'src, 'token>> {
        let prefix: PrefixParselet<'src, 'token> = match kind {
            IDENT => Self::var_ref,
            T![if] => Self::if_expr,
            // T![match] => Self::match_expr,
            // T![while] => Self::while_expr,
            // T![loop] => Self::loop_expr,
            // T![for] => Self::for_expr,
            T![return] => Self::return_expr,
            T![break] => Self::break_expr,
            T![continue] => Self::continue_expr,
            T!['('] => Self::parens_or_tuple,
            T!['{'] => Self::block_expr,
            T![-] | T![!] => Self::unary_expr,
            T![|] => Self::closure,

            // TODO: Floats
            NUMBER_LITERAL | STRING_LITERAL | CHAR_LITERAL | T![true] | T![false] => {
                Self::literal_expr
            }

            _ => return None,
        };

        Some(prefix)
    }

    fn expr_infix(kind: SyntaxKind) -> Option<InfixParselet<'src, 'token>> {
        let infix: InfixParselet<'src, 'token> = match kind {
            // T!['['] => Self::index_array,
            // T![as] => Self::as_cast,
            T![.] => Self::method_or_access,

            // TODO: Powers?
            T![+]
            | T![-]
            | T![*]
            | T![/]
            | T![%]
            | T![&]
            | T![|]
            | T![<<]
            | T![>>]
            | T![and]
            | T![or]
            | T![<]
            | T![>]
            | T![<=]
            | T![>=]
            | T![==]
            | T![!=] => Self::binop,

            // TODO: Powers?
            T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T![%=]
            | T![<<=]
            | T![>>=]
            | T![|=]
            | T![&=]
            | T![^=] => Self::assign,

            _ => return None,
        };

        Some(infix)
    }

    fn expr_postfix(kind: SyntaxKind) -> Option<PostfixParselet<'src, 'token>> {
        let postfix: PostfixParselet<'src, 'token> = match kind {
            T!['('] => Self::function,
            // T![..] => Self::ranges,
            // T!['['] => Self::index_array,
            // T![as] => Self::as_cast,
            T![.] => Self::method_or_access,

            // TODO: Powers?
            T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T![%=]
            | T![<<=]
            | T![>>=]
            | T![|=]
            | T![&=]
            | T![^=] => Self::assign,

            _ => return None,
        };

        Some(postfix)
    }

    fn expr_precedence(&mut self) -> u8 {
        ExprPrecedence::try_from(self.current())
            .map(ExprPrecedence::precedence)
            .unwrap_or(0)
    }
}

impl Parser<'_, '_> {
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
    pub(super) fn parens_or_tuple(&mut self, _paren: SyntaxKind) -> Option<CompletedMarker> {
        let marker = self.start();

        if !self.expect(T!['(']) {
            marker.abandon(self);
            return None;
        }

        let first_expr = self.start();
        // TODO: Error handling
        self.expr();

        let kind = if self.try_expect(T![,]) {
            first_expr.complete(self, TUPLE_EXPR_ELEM);

            while !self.at(T![')']) {
                let tuple_elem = self.start();

                // TODO: Error handling
                self.expr();

                if !self.try_expect(T![,]) {
                    let span = self.current_span();
                    let error = Diagnostic::error()
                        .with_message("missing comma between tuple elements")
                        .with_label(Label::primary(span).with_message("expected a comma"));

                    self.error(error);
                }

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

    // test(expr) var_ref
    // - a
    // - abcd
    // - abcd1245
    // - _
    // - _135
    // - _dsg434
    pub(super) fn var_ref(&mut self, _ident: SyntaxKind) -> Option<CompletedMarker> {
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
    pub(super) fn closure(&mut self, _pipe: SyntaxKind) -> Option<CompletedMarker> {
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

    // TODO: Floats
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
    fn literal_expr(&mut self, kind: SyntaxKind) -> Option<CompletedMarker> {
        let literal = self.start();

        match kind {
            NUMBER_LITERAL => {
                self.bump();
                Some(literal.complete(self, NUMBER))
            }

            STRING_LITERAL => {
                self.bump();
                Some(literal.complete(self, STRING))
            }

            CHAR_LITERAL => {
                self.bump();
                Some(literal.complete(self, CHAR))
            }

            T![true] | T![false] => {
                self.bump();
                Some(literal.complete(self, BOOL))
            }

            kind => unreachable!("invalid literal kind {:?}", kind),
        }
    }

    // test(expr) negation
    // - --(-100)
    // test(expr) boolean_not
    // - !!(!true)
    fn unary_expr(&mut self, operator: SyntaxKind) -> Option<CompletedMarker> {
        let marker = self.start();

        // Eat the operator’s token.
        let operand = self.start();
        self.expect(operator);
        operand.complete(self, UNARY_OP);

        // TODO: Improve error handling
        let expr_invalid = self.expr().is_none();

        let complete = marker.complete(self, UNARY_EXPR);
        if expr_invalid {
            return None;
        }

        Some(complete)
    }

    fn block_expr(&mut self, _bracket: SyntaxKind) -> Option<CompletedMarker> {
        self.block(false)
    }

    fn return_expr(&mut self, _return: SyntaxKind) -> Option<CompletedMarker> {
        let ret = self.start();

        self.expect(T![return]);
        self.expr();

        Some(ret.complete(self, RET_EXPR))
    }

    fn continue_expr(&mut self, _continue: SyntaxKind) -> Option<CompletedMarker> {
        let cont = self.start();

        self.expect(T![continue]);
        self.expr();

        Some(cont.complete(self, CONTINUE_EXPR))
    }

    fn break_expr(&mut self, _break: SyntaxKind) -> Option<CompletedMarker> {
        let brk = self.start();

        self.expect(T![break]);
        self.expr();

        Some(brk.complete(self, BREAK_EXPR))
    }

    fn if_expr(&mut self, _if: SyntaxKind) -> Option<CompletedMarker> {
        self.parse_if()
    }

    fn binop(&mut self, _op: SyntaxKind, lhs: CompletedMarker) -> Option<CompletedMarker> {
        let op = self.start();
        self.bump();
        op.complete(self, BIN_OP);

        // TODO: Error handling for the right hand side
        let rhs = lhs.precede(self);
        self.expr();
        Some(rhs.complete(self, BIN_EXPR))
    }

    fn assign(&mut self, _assign: SyntaxKind, rvalue: CompletedMarker) -> Option<CompletedMarker> {
        let assign = self.start();
        self.bump();
        assign.complete(self, ASSIGN_OP);

        let lvalue = rvalue.precede(self);
        self.expr();
        Some(lvalue.complete(self, ASSIGN))
    }

    fn method_or_access(
        &mut self,
        _dot: SyntaxKind,
        callee: CompletedMarker,
    ) -> Option<CompletedMarker> {
        let marker = callee.precede(self);
        self.expect(T![.]);

        // Tuple access
        Some(if self.at(NUMBER_LITERAL) {
            let accessor = self.start();
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
        })
    }

    fn function(&mut self, _paren: SyntaxKind, callee: CompletedMarker) -> Option<CompletedMarker> {
        let call = callee.precede(self);
        self.expect(T!['(']);

        while !self.at(T![')']) {
            let arg = self.start();
            self.expr();

            self.try_expect(T![,]);

            arg.complete(self, FUNCTION_CALL_ARG);
        }
        self.expect(T![')']);

        Some(call.complete(self, FUNCTION_CALL))
    }
}
