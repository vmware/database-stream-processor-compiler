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
    // test extern_function
    // - extern function foo() {}
    // test_err double_extern_function
    // - extern extern function foo() {}
    // test_err output_function
    // - extern output function foo() {}
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

    // test_err relation_with_input_and_output
    // - input output relation Foo()
    // test_err input_function
    // - extern input function foo() {}
    // test_err extern_relation
    // - input extern relation Foo()
    // - input extern extern relation Foo()
    // - input extern multiset Foo()
    // - input extern extern multiset Foo()
    // - input extern stream Foo()
    // - input extern extern stream Foo()
    fn check_relation(&mut self, relation: &RelationDef, ctx: &mut RuleCtx) -> Option<()> {
        let (mut first_input, mut first_output) = (None, None);
        let relation_kind = match relation.keyword().as_deref() {
            Some(RelKw::Multiset(_)) => "multiset",
            Some(RelKw::Stream(_)) => "stream",
            Some(RelKw::Relation(_)) | None => "relation",
        };

        let modifiers = relation.modifiers()?;
        for modifier in modifiers.modifiers() {
            match &*modifier {
                Modifier::Extern(ext) => {
                    let span = ext.span(ctx.file_id);
                    let error = Diagnostic::error()
                        .with_message(format!(
                            "received an 'extern' modifier on a {} definition",
                            relation_kind,
                        ))
                        .with_label(Label::primary(span).with_message(format!(
                            "{}s cannot be marked as 'extern'",
                            relation_kind,
                        )));

                    ctx.diagnostics.push(error);
                }

                Modifier::Input(input) => {
                    let span = input.span(ctx.file_id);

                    if let Some(first_input) = first_input {
                        let error = Diagnostic::error()
                            .with_message(
                                "received multiple 'input' modifiers where one was expected",
                            )
                            .with_label(Label::primary(span).with_message(format!(
                                "{}s can only have a single 'input' modifier",
                                relation_kind,
                            )))
                            .with_label(
                                Label::secondary(first_input)
                                    .with_message("first 'input' given here"),
                            );

                        ctx.diagnostics.push(error);
                    } else {
                        first_input = Some(span);

                        if let Some(output) = first_output {
                            let error = Diagnostic::error()
                                .with_message(format!(
                                    "a {} cannot be marked as both 'input' and 'output'",
                                    relation_kind,
                                ))
                                .with_label(
                                    Label::primary(output).with_message("marked as 'output' here"),
                                )
                                .with_label(
                                    Label::secondary(span).with_message("marked as 'input' here"),
                                );

                            ctx.diagnostics.push(error);
                        }
                    }
                }

                Modifier::Output(output) => {
                    let span = output.span(ctx.file_id);

                    if let Some(first_output) = first_output {
                        let error = Diagnostic::error()
                            .with_message(
                                "received multiple 'output' modifiers where one was expected",
                            )
                            .with_label(Label::primary(span).with_message(format!(
                                "{}s can only have a single 'output' modifier",
                                relation_kind,
                            )))
                            .with_label(
                                Label::secondary(first_output)
                                    .with_message("first 'output' given here"),
                            );

                        ctx.diagnostics.push(error);
                    } else {
                        first_output = Some(span);

                        if let Some(input) = first_input {
                            let error = Diagnostic::error()
                                .with_message(format!(
                                    "a {} cannot be marked as both 'input' and 'output'",
                                    relation_kind,
                                ))
                                .with_label(
                                    Label::primary(input).with_message("marked as 'input' here"),
                                )
                                .with_label(
                                    Label::secondary(span).with_message("marked as 'output' here"),
                                );

                            ctx.diagnostics.push(error);
                        }
                    }
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
                RelationDef(relation) => self.check_relation(&*relation, ctx),

                _ => Some(()),
            }
        }
    }
}