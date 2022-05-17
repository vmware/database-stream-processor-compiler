use crate::{ast::nodes::StructDef, AstVisitor, RuleCtx, SyntaxNode, SyntaxNodeExt};
use ddlog_diagnostics::{Diagnostic, Label, Span};

// We allow a maximum of (2^16)-1 struct fields in a single struct
const MAX_FIELDS: usize = u16::MAX as usize;

#[derive(Default)]
pub struct StructFieldCount;

impl AstVisitor for StructFieldCount {
    fn check_node(&mut self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        let strct = node.try_to::<StructDef>()?;
        let fields = strct.fields()?;
        let total_fields = fields.len();

        if total_fields > MAX_FIELDS {
            let span = Span::from_text_range(strct.signature_span(), ctx.file());

            let note = if let Some(name) = strct.ident() {
                format!(
                    "ddlog allows a maximum of {MAX_FIELDS} struct fields, but {} has {total_fields}",
                    ctx.interner().resolve(name),
                )
            } else {
                format!(
                    "ddlog allows a maximum of {MAX_FIELDS} struct fields, but the current struct has {total_fields}",
                )
            };

            let error = Diagnostic::error()
                .with_message(format!("struct contains more than {MAX_FIELDS} fields"))
                .with_label(Label::primary(span).with_message(note));

            ctx.push_diagnostic(error);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{StructFieldCount, MAX_FIELDS};
    use crate::validation::tests;
    use std::fmt::Write;

    #[test]
    fn too_many_struct_fields() {
        let mut source = String::from("struct Foo {\n");
        for i in 0..MAX_FIELDS + 1 {
            source.push_str("    field_");
            write!(source, "{i}").unwrap();
            source.push_str(": (),\n");
        }
        source.push_str("}\n");

        tests::validate_fail(
            &source,
            "too_many_struct_fields",
            StructFieldCount::default(),
        );
    }

    #[test]
    fn max_struct_fields() {
        let mut source = String::from("struct Foo {\n");
        for i in 0..MAX_FIELDS {
            source.push_str("    field_");
            write!(source, "{i}").unwrap();
            source.push_str(": (),\n");
        }
        source.push_str("}\n");

        tests::validate_pass(&source, "max_struct_fields", StructFieldCount::default());
    }
}
