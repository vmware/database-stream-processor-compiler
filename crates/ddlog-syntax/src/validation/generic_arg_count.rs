use crate::{
    ast::{nodes::Generics, AstNode},
    AstVisitor, RuleCtx, SyntaxNode, SyntaxNodeExt,
};
use ddlog_diagnostics::{Diagnostic, Label};

// We allow a maximum of 255 generics on a single item
const MAX_GENERICS: usize = u8::MAX as usize;

#[derive(Default)]
pub struct GenericArgCount;

impl AstVisitor for GenericArgCount {
    fn check_node(&mut self, node: &SyntaxNode, ctx: &mut RuleCtx) -> Option<()> {
        let generics = node.try_to::<Generics>()?;
        let total_generics = generics.generics().count();

        if total_generics > MAX_GENERICS {
            let span = generics.span(ctx.file());
            let note = format!(
                "ddlog allows a maximum of {MAX_GENERICS} generics but {total_generics} were provided",
            );

            let error = Diagnostic::error()
                .with_message(format!(
                    "more than {MAX_GENERICS} generic args were provided",
                ))
                .with_label(Label::primary(span).with_message(note));
            ctx.push_diagnostic(error);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{GenericArgCount, MAX_GENERICS};
    use crate::validation::tests;
    use std::fmt::Write;

    #[test]
    fn too_many_struct_generics() {
        let mut source = String::from("struct Foo<\n");
        for i in 0..MAX_GENERICS + 1 {
            source.push_str("    T");
            write!(source, "{i}").unwrap();
            source.push_str(",\n");
        }
        source.push_str(">();\n");

        tests::validate_fail(
            &source,
            "too_many_struct_generics",
            GenericArgCount::default(),
        );
    }

    #[test]
    fn max_struct_generics() {
        let mut source = String::from("struct Foo<\n");
        for i in 0..MAX_GENERICS {
            source.push_str("    T");
            write!(source, "{i}").unwrap();
            source.push_str(",\n");
        }
        source.push_str(">();\n");

        tests::validate_pass(&source, "max_struct_generics", GenericArgCount::default());
    }

    #[test]
    fn too_many_fn_generics() {
        let mut source = String::from("fn foo<\n");
        for i in 0..MAX_GENERICS + 1 {
            source.push_str("    T");
            write!(source, "{i}").unwrap();
            source.push_str(",\n");
        }
        source.push_str(">() {}\n");

        tests::validate_fail(&source, "too_many_fn_generics", GenericArgCount::default());
    }

    #[test]
    fn max_fn_generics() {
        let mut source = String::from("fn foo<\n");
        for i in 0..MAX_GENERICS {
            source.push_str("    T");
            write!(source, "{i}").unwrap();
            source.push_str(",\n");
        }
        source.push_str(">() {}\n");

        tests::validate_pass(&source, "max_fn_generics", GenericArgCount::default());
    }

    /*
    // FIXME: Enable this once enums are parsed
    #[test]
    fn too_many_enum_generics() {
        let mut source = String::from("enum Foo<\n");
        for i in 0..MAX_GENERICS + 1 {
            source.push_str(&format!("    T{i},\n"));
        }
        source.push_str("> {}\n");

        tests::validate_fail(
            &source,
            "too_many_enum_generics",
            GenericArgCount::default(),
        );
    }

    #[test]
    fn max_enum_generics() {
        let mut source = String::from("enum Foo<\n");
        for i in 0..MAX_GENERICS {
            source.push_str(&format!("    T{i},\n"));
        }
        source.push_str("> {}\n");

        tests::validate_pass(
            &source,
            "max_enum_generics",
            GenericArgCount::default(),
        );
    }
    */
}
