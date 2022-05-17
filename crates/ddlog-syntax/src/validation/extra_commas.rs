use crate::{
    ast::{
        nodes::{Attributes, FunctionArgs, RelationDef},
        tokens::RelKw,
        AstNode, AstToken,
    },
    visitor::RuleCtx,
    AstVisitor, SyntaxNode, SyntaxNodeExt,
};
use ddlog_diagnostics::{Diagnostic, Label};

/// Ensure that comma'd things (attributes, function arguments, relation
/// columns, etc.) have the proper number of commas delimiting them
#[derive(Default)]
pub struct ExtraCommas;

impl ExtraCommas {
    // test attributes_with_proper_commas
    // - #[foo = bar]
    // - #[foo = bar,]
    // - #[foo = bar, bar = baz, bing = bop,]
    // - #[foo = bar, bar = baz, bing = bop]
    // - fn foo() {}
    fn check_attributes(&self, attributes: &Attributes, ctx: &mut RuleCtx) -> Option<()> {
        for attribute in attributes.attributes() {
            let total_pairs = attribute.attr_pairs().count();
            for (pair_idx, pair) in attribute.attr_pairs().enumerate() {
                let total_commas = pair.commas().count();

                // If there's no commas where commas were expected, error
                // test_err missing_attribute_commas
                // - #[foo = bar bar = baz]
                // - fn foo() {}
                if total_commas == 0 && pair_idx + 1 != total_pairs {
                    let span = pair.syntax().trimmed_span(ctx.file_id).end_span();

                    let error = Diagnostic::error()
                        .with_message("missing comma between attribute pairs")
                        .with_label(Label::primary(span).with_message("expected a comma here"));

                    ctx.diagnostics.push(error);

                // if there's too many commas, error
                // test_err too_many_attribute_commas
                // - #[foo = bar,, bar = baz]
                // - fn foo() {}
                } else if total_commas > 1 {
                    // Gather the spans of the extraneous commas
                    let span = pair
                        .commas()
                        .skip(1)
                        .map(|comma| comma.span(ctx.file_id))
                        .reduce(|acc, span| acc.merge(span))?;

                    // Properly pluralize the message
                    let message = if total_commas == 2 {
                        "remove this comma"
                    } else {
                        "remove these commas"
                    };
                    let error = Diagnostic::error()
                        .with_message("too many commas between attribute pairs")
                        .with_label(Label::primary(span).with_message(message));

                    ctx.diagnostics.push(error);
                }
            }
        }

        Some(())
    }

    // test function_args_with_proper_commas
    // - fn foo(bar: Baz) {}
    // - fn foo(bar: Baz,) {}
    // - fn foo(bar: Baz, foo: Bar, bing: Bop) {}
    // - fn foo(bar: Baz, foo: Bar, bing: Bop,) {}
    fn check_function_args(&mut self, args: &FunctionArgs, ctx: &mut RuleCtx) -> Option<()> {
        let total_args = args.args().count();
        for (pair_idx, arg) in args.args().enumerate() {
            let total_commas = arg.commas().count();

            // If there's no commas where commas were expected, error
            // test_err missing_function_arg_commas
            // - fn foo(foo: Bar bar: Baz) {}
            if total_commas == 0 && pair_idx + 1 != total_args {
                let span = arg.syntax().trimmed_span(ctx.file_id).end_span();

                let error = Diagnostic::error()
                    .with_message("missing comma between function arguments")
                    .with_label(Label::primary(span).with_message("expected a comma here"));

                ctx.diagnostics.push(error);

            // if there's too many commas, error
            // test_err too_many_function_arg_commas
            // - fn foo(foo: Bar,, bar: Baz) {}
            } else if total_commas > 1 {
                // Gather the spans of the extraneous commas
                let span = arg
                    .commas()
                    .skip(1)
                    .map(|comma| comma.span(ctx.file_id))
                    .reduce(|acc, span| acc.merge(span))?;

                // Properly pluralize the message
                let message = if total_commas == 2 {
                    "remove this comma"
                } else {
                    "remove these commas"
                };
                let error = Diagnostic::error()
                    .with_message("too many commas between function arguments")
                    .with_label(Label::primary(span).with_message(message));

                ctx.diagnostics.push(error);
            }
        }

        Some(())
    }
}

impl AstVisitor for ExtraCommas {
    fn check_node(&mut self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        tracing::trace!(
            visitor = "ModifierValidator",
            node = %node.debug(ctx.interner(), true),
            "checking node",
        );

        match_ast! {
            match node {
                Attributes(attributes) => self.check_attributes(&*attributes, ctx),
                FunctionArgs(args) => self.check_function_args(&*args, ctx),
                // TODO: Struct & enum defs

                _ => Some(()),
            }
        }
    }
}
