use crate::{
    ast::{
        nodes::{FunctionDef, RelationDef},
        tokens::{Modifier, RelKw},
        AstToken,
    },
    visitor::RuleCtx,
    AstVisitor, SyntaxNode,
};
use ddlog_diagnostics::{Diagnostic, Label};

/// Ensure that the proper attributes are on declarations
///
/// - Functions may have zero or one `extern` modifiers
/// - Functions cannot have any `input` or `output` modifiers
/// - Relations (of any kind) may have zero or one `input` or `output` modifiers
/// - Relations (of any kind) cannot have any `extern` modifiers
/// - Relations (of any kind) cannot have both an `input` and an `output` modifier,
///   the two modifier kinds are exclusive
#[derive(Default)]
pub struct ModifierValidator;

impl ModifierValidator {
    // test pub_function
    // - pub fn foo() {}
    fn check_function(&mut self, function: &FunctionDef, ctx: &mut RuleCtx) -> Option<()> {
        let mut first_extern = None;

        let modifiers = function.modifiers()?;
        for modifier in modifiers.modifiers() {
            match &*modifier {
                Modifier::Extern(ext) => {
                    let span = ext.span(ctx.file_id);

                    if let Some(first_extern) = first_extern {
                        let error =
                            Diagnostic::error()
                                .with_message(
                                    "received multiple 'extern' modifiers where one was expected",
                                )
                                .with_label(Label::primary(span).with_message(
                                    "functions can only have a single 'extern' modifier",
                                ))
                                .with_label(
                                    Label::secondary(first_extern)
                                        .with_message("first 'extern' given here"),
                                );

                        ctx.diagnostics.push(error);
                    } else {
                        first_extern = Some(span);
                    }
                }

                Modifier::Input(input) => {
                    let span = input.span(ctx.file_id);
                    let error = Diagnostic::error()
                        .with_message("received an 'input' modifier on a function definition")
                        .with_label(
                            Label::primary(span)
                                .with_message("functions cannot be marked as 'input'"),
                        );

                    ctx.diagnostics.push(error);
                }

                Modifier::Output(output) => {
                    let span = output.span(ctx.file_id);
                    let error = Diagnostic::error()
                        .with_message("received an 'output' modifier on a function definition")
                        .with_label(
                            Label::primary(span)
                                .with_message("functions cannot be marked as 'output'"),
                        );

                    ctx.diagnostics.push(error);
                }
            }
        }

        Some(())
    }
}

impl AstVisitor for ModifierValidator {
    fn check_node(&mut self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        match_ast! {
            match node {
                FunctionDef(function) => self.check_function(&*function, ctx),
                // TODO: Struct, enum, type and const defs

                _ => Some(()),
            }
        }
    }
}
