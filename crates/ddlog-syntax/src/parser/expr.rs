use crate::{
    parser::{precedence::ExprPrecedence, CompletedMarker, Marker, Parser},
    SyntaxKind::{
        self, ASSIGN, ASSIGN_OP, BIN_EXPR, BIN_OP, BLOCK, BOOL, BREAK_EXPR, CHAR, CHAR_LITERAL,
        CLOSURE_ARG, CLOSURE_EXPR, CONTINUE_EXPR, ELSE_BLOCK, FIELD_ACCESS, FIELD_ACCESSOR_NAME,
        FUNCTION_CALL, FUNCTION_CALL_ARG, IDENT, IF_BLOCK, IF_EXPR, METHOD_CALL, METHOD_CALL_ARG,
        NUMBER, NUMBER_LITERAL, PAREN_EXPR, RET_EXPR, STRING, STRING_LITERAL, TUPLE_EXPR_ELEM,
        TUPLE_INIT_EXPR, UNARY_EXPR, UNARY_OP, VAR_REF,
    },
    TokenSet,
};
use ddlog_diagnostics::{Diagnostic, Label};
use std::convert::TryFrom;

type PrefixParselet<'src, 'token> = fn(&mut Parser<'src, 'token>) -> Option<CompletedMarker>;
type PostfixParselet<'src, 'token> =
    fn(&mut Parser<'src, 'token>, CompletedMarker) -> Option<CompletedMarker>;
type InfixParselet<'src, 'token> =
    fn(&mut Parser<'src, 'token>, CompletedMarker) -> Option<CompletedMarker>;

pub(super) const EXPR_RECOVERY_SET: TokenSet = token_set! {
    T![')'],
    T!['}'],
    T![;],
};

// TODO: Allow `let x = y` to be an expression to allow assignments within clauses, e.g.
// ```
// R(x) :- A(b), let x = b * b;
// ```
impl<'src, 'token> Parser<'src, 'token> {
    // test(expr) negation_has_higher_binding_power_than_infix
    // - -20 + 20
    // test(expr) parentheses_affect_precedence
    // - 5 * (2 + 1)
    pub(super) fn expr(&mut self) -> Option<CompletedMarker> {
        self.expr_inner(0)
    }

    /// The innards of [`Parser::expr()`], does all of the actual work
    ///
    /// Utilizes [pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
    fn expr_inner(&mut self, precedence: u8) -> Option<CompletedMarker> {
        let token = self.peek();

        let prefix = Self::expr_prefix(token);
        if let Some(prefix) = prefix {
            let mut left = prefix(self)?;

            if let Some(peek) = self.try_peek() {
                let postfix = Self::expr_postfix(peek);

                if let Some(postfix) = postfix {
                    left = postfix(self, left)?;
                }
            }

            while precedence < self.expr_precedence() {
                // TODO: Handle EOF here?
                let token = self.peek();

                let infix = Self::expr_infix(token);
                if let Some(infix) = infix {
                    left = infix(self, left)?;
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
            T!['{'] => Self::block,
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
            T!['('] => Self::function_call,
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

    /// Gets the precedence of the current token
    fn expr_precedence(&mut self) -> u8 {
        ExprPrecedence::try_from(self.current())
            .map(ExprPrecedence::precedence)
            .unwrap_or(0)
    }
}

impl Parser<'_, '_> {
    // test(expr) assign
    // - x = 10
    // test(expr) assign_rvalue
    // - x.y = 100
    // test(expr) assign_exotic
    // - {
    // -     x += y;
    // -     x -= y;
    // -     x /= y;
    // -     x *= y;
    // -     x %= y;
    // -     x ^= y;
    // -     x &= y;
    // -     x |= y;
    // - }
    fn assign(&mut self, rvalue: CompletedMarker) -> Option<CompletedMarker> {
        let assign = self.start();
        self.bump();
        assign.complete(self, ASSIGN_OP);

        let lvalue = rvalue.precede(self);
        self.expr();
        Some(lvalue.complete(self, ASSIGN))
    }

    // test(expr) binary_ops
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
    // -     foo + bar;
    // -     foo and bar;
    // -     foo & bar;
    // -     foo % bar;
    // - }
    fn binop(&mut self, lhs: CompletedMarker) -> Option<CompletedMarker> {
        let op = self.start();
        self.bump();
        op.complete(self, BIN_OP);

        // TODO: Error handling for the right hand side
        let rhs = lhs.precede(self);
        self.expr();
        Some(rhs.complete(self, BIN_EXPR))
    }

    // test(expr) toilet
    // - || {}
    // test(expr) identity
    // - |x| x
    // test(expr) block_closure
    // - |foo, bar| {
    // -     return baz(foo + bar);
    // - }
    pub(super) fn closure(&mut self) -> Option<CompletedMarker> {
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

    // test(expr) func_call
    // - foo(bar, baz, bing, bop)
    fn function_call(&mut self, callee: CompletedMarker) -> Option<CompletedMarker> {
        let call = callee.precede(self);
        self.expect(T!['(']);

        while !self.at(T![')']) {
            let arg = self.start();
            self.expr();
            self.eat_commas();

            arg.complete(self, FUNCTION_CALL_ARG);
        }
        self.expect(T![')']);

        Some(call.complete(self, FUNCTION_CALL))
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

    // test(expr) if_expr
    // - if x {
    // -     x + y;
    // - } else if y {
    // -     x + y;
    // - } else if z {
    // -     x + y;
    // - } else {
    // -     x + y;
    // - }
    // test(expr) two_ifs
    // - if x {
    // -     x + y;
    // - }
    // - if y {
    // -     x + y;
    // - } else if z {
    // -     x + y;
    // - } else {
    // -     x + y;
    // - }
    pub(super) fn if_expr(&mut self) -> Option<CompletedMarker> {
        let if_stmt = self.start();

        self.if_block(None);
        while self.at(T![else]) {
            let block = self.start();
            self.expect(T![else]);

            if self.at(T![if]) {
                self.if_block(Some(block));
            } else {
                self.else_block(Some(block));
            }
        }

        if self.at(T![else]) {
            let block = self.start();
            self.expect(T![else]);

            self.else_block(Some(block));
        }

        Some(if_stmt.complete(self, IF_EXPR))
    }

    fn if_block(&mut self, block: Option<Marker>) -> Option<CompletedMarker> {
        let block = block.unwrap_or_else(|| self.start());

        self.expect(T![if]);
        self.expr();
        self.block();

        Some(block.complete(self, IF_BLOCK))
    }

    fn else_block(&mut self, block: Option<Marker>) -> Option<CompletedMarker> {
        let block = block.unwrap_or_else(|| self.start());

        self.block();

        Some(block.complete(self, ELSE_BLOCK))
    }

    // test(expr) block_exprs
    // - { 10 } - {{ 5 + ({ 99 }) }}
    // test_err(expr) unclosed_block
    // - {{ 10 }
    pub(super) fn block(&mut self) -> Option<CompletedMarker> {
        let block = self.start();
        if !self.expect(T!['{']) {
            block.abandon(self);
            return None;
        }

        // FIXME: Statements, not expressions
        let mut did_error = false;
        while !self.at(T!['}']) && !self.at_end() {
            if self.stmt().is_none() {
                // Bump so we don't loop forever
                self.error_eat_until(token_set! { T!['}'], T![;] });
                did_error = true;

                break;
            }
        }

        self.expect(T!['}']);

        let marker = block.complete(self, BLOCK);
        if did_error {
            None
        } else {
            Some(marker)
        }
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
    fn literal_expr(&mut self) -> Option<CompletedMarker> {
        let literal = self.start();

        match self.current() {
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

    // test(expr) method_call
    // - foo.bar()
    // test(expr) method_call_args
    // - foo.bar(baz, bing, bong)
    // test(expr) method_call_args_trailing
    // - foo.bar(baz, bing, bong,)
    // test(expr) tuple_access
    // - foo.1
    // test(expr) tuple_access_2
    // - (foo,).0
    // test(expr) field_access
    // - foo.bar
    // test(expr) chained_field_access
    // - foo.bar.baz.bing.bop
    fn method_or_access(&mut self, callee: CompletedMarker) -> Option<CompletedMarker> {
        let marker = callee.precede(self);
        self.expect(T![.]);

        // Tuple access
        Some(if self.at(NUMBER_LITERAL) {
            let number = self.start();
            self.expect(NUMBER_LITERAL);
            number.complete(self, NUMBER);
            marker.complete(self, FIELD_ACCESS)

        // Field access or method calls
        } else {
            let accessor = self.start();
            self.var_ref();

            // Method call
            if self.try_expect(T!['(']) {
                accessor.abandon(self);

                while !self.at(T![')']) {
                    let arg = self.start();

                    // TODO: Error handling
                    self.expr().unwrap();
                    self.eat_commas();

                    arg.complete(self, METHOD_CALL_ARG);
                }

                self.expect(T![')']);
                marker.complete(self, METHOD_CALL)

            // Field access
            } else {
                accessor.complete(self, FIELD_ACCESSOR_NAME);
                marker.complete(self, FIELD_ACCESS)
            }
        })
    }

    // test(expr) parenthesised
    // - (((((((((2 + (((((5))))))))))))))
    // test_err(expr) unclosed_paren
    // - (((100))
    // test(expr) parenthesised_literal
    // - (1)
    // test(expr) single_element_tuple
    // - (1,)
    // test(expr) tuple_literal_1
    // - (1,)
    // test(expr) tuple_literal_2
    // - (1, 2)
    // test(expr) tuple_literal_2_trailing
    // - (1, 2,)
    // test(expr) tuple_literal_3
    // - (1, 2, 3)
    // test(expr) tuple_literal_3_trailing
    // - (1, 2, 3,)
    // test_err(expr) missing_tuple_comma
    // - (1, 2 3)
    // test(expr) unit_literal
    // - ()
    pub(super) fn parens_or_tuple(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        if !self.expect(T!['(']) {
            marker.abandon(self);
            return None;
        }

        // We special case unit literals
        if self.at(T![')']) {
            self.expect(T![')']);
            // TODO: Should this be a unit literal instead of a tuple expr?
            return Some(marker.complete(self, TUPLE_INIT_EXPR));
        }

        let first_expr = self.start();
        // TODO: Error handling
        self.expr().unwrap();

        let kind = if self.try_expect(T![,]) {
            first_expr.complete(self, TUPLE_EXPR_ELEM);

            while !self.at(T![')']) {
                let tuple_elem = self.start();

                // TODO: Error handling
                self.expr();

                if !self.try_expect(T![,]) && !self.at(T![')']) {
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

    // test(expr) continue
    // - continue
    // test(expr) continue_expr
    // - continue foo + bar
    // test(expr) continue_chain
    // - continue continue continue continue
    fn continue_expr(&mut self) -> Option<CompletedMarker> {
        let cont = self.start();

        self.expect(T![continue]);
        self.expr();

        Some(cont.complete(self, CONTINUE_EXPR))
    }

    // test(expr) break
    // - break
    // test(expr) break_expr
    // - break foo + bar + 1
    // test(expr) break_chain
    // - break break break break
    fn break_expr(&mut self) -> Option<CompletedMarker> {
        let brk = self.start();

        self.expect(T![break]);
        self.expr();

        Some(brk.complete(self, BREAK_EXPR))
    }

    // test(expr) return
    // - return
    // test(expr) return_expr
    // - return foo + bar + 1
    // test(expr) return_chain
    // - return return return return
    fn return_expr(&mut self) -> Option<CompletedMarker> {
        let ret = self.start();

        self.expect(T![return]);
        self.expr();

        Some(ret.complete(self, RET_EXPR))
    }

    // test(expr) infix_negation
    // - -10
    // test(expr) negation
    // - --(-100)
    // test(expr) boolean_not
    // - !!(!true)
    fn unary_expr(&mut self) -> Option<CompletedMarker> {
        let marker = self.start();

        // Eat the operator’s token.
        let operand = self.start();
        self.bump();
        operand.complete(self, UNARY_OP);

        // TODO: Improve error handling
        let expr_invalid = self.expr().is_none();

        let complete = marker.complete(self, UNARY_EXPR);
        if expr_invalid {
            return None;
        }

        Some(complete)
    }

    // test(expr) var_ref
    // - {
    // -     a;
    // -     abcd;
    // -     abcd1245;
    // -     _;
    // -     _135;
    // -     _dsg434;
    // - }
    pub(super) fn var_ref(&mut self) -> Option<CompletedMarker> {
        let var = self.start();
        self.ident();
        Some(var.complete(self, VAR_REF))
    }
}
