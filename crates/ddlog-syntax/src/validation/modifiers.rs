use crate::{
    ast::{
        nodes::{FuncDef, RelationDef},
        support,
        tokens::{Extern, Input, Multiset, Output, Stream},
        AstNode,
    },
    visitor::RuleCtx,
    AstVisitor, SyntaxElementRef, SyntaxNode, SyntaxTokenExt,
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
    fn check_function(&mut self, function: &FuncDef, ctx: &mut RuleCtx) -> Option<()> {
        let mut first_extern = None;

        for modifier in function.modifiers()?.syntax().children_with_tokens() {
            if let SyntaxElementRef::Token(token) = modifier {
                if token.is::<Extern>() {
                    let span = token.span(ctx.file_id);

                    if let Some(first_extern) = first_extern {
                        tracing::trace!(
                            %first_extern,
                            current_extern = %span,
                            "found excess extern modifier on function",
                        );

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
                } else if token.is::<Input>() {
                    let span = token.span(ctx.file_id);
                    tracing::trace!(
                        output = %span,
                        "found invalid output modifier on function",
                    );

                    let error = Diagnostic::error()
                        .with_message("received an 'input' modifier on a function definition")
                        .with_label(
                            Label::primary(span)
                                .with_message("functions cannot be marked as 'input'"),
                        );

                    ctx.diagnostics.push(error);
                } else if token.is::<Output>() {
                    let span = token.span(ctx.file_id);
                    tracing::trace!(
                        input = %span,
                        "found invalid input modifier on function",
                    );

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

    fn check_relation(&mut self, relation: &RelationDef, ctx: &mut RuleCtx) -> Option<()> {
        let (mut first_input, mut first_output) = (None, None);
        // FIXME: Fix codegen for `RelationDef::keyword()`
        let relation_kind = if support::token::<Multiset>(relation.syntax()).is_some() {
            "multiset"
        } else if support::token::<Stream>(relation.syntax()).is_some() {
            "stream"
        } else {
            "relation"
        };

        for modifier in relation.modifiers()?.syntax().children_with_tokens() {
            if let SyntaxElementRef::Token(token) = modifier {
                if token.is::<Extern>() {
                    let span = token.span(ctx.file_id);
                    tracing::trace!(
                        _extern = %span,
                        "found invalid extern modifier on {}",
                        relation_kind,
                    );

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
                } else if token.is::<Input>() {
                    let span = token.span(ctx.file_id);

                    if let Some(first_input) = first_input {
                        tracing::trace!(
                            %first_input,
                            current_input = %span,
                            "found excess input modifier on {}",
                        relation_kind,
                        );

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
                } else if token.is::<Output>() {
                    let span = token.span(ctx.file_id);

                    if let Some(first_output) = first_output {
                        tracing::trace!(
                            %first_output,
                            current_output = %span,
                            "found excess output modifier on relation",
                        );

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
        tracing::trace!(
            visitor = "ModifierValidator",
            node = %node.debug(ctx.interner(), true),
            "checking node",
        );

        match_ast! {
            match node {
                FuncDef(function) => self.check_function(&*function, ctx),
                RelationDef(relation) => self.check_relation(&*relation, ctx),

                _ => Some(()),
            }
        }
    }
}
