// mod extra_commas;
// mod modifiers;
mod generic_arg_count;
mod struct_field_count;

use crate::{
    visitor::{self, RuleCtx},
    AstVisitor, SyntaxNode,
};

pub fn run_default_validators(node: &SyntaxNode, ctx: &mut RuleCtx) {
    let mut validators = validators::default();

    run_validators(node, ctx, &mut validators)
}

pub fn run_validators<I, V>(node: &SyntaxNode, ctx: &mut RuleCtx, validators: I)
where
    I: IntoIterator<Item = V>,
    V: AstVisitor,
{
    validators
        .into_iter()
        .for_each(|mut validator| visitor::apply_visitor(node, &mut validator, ctx));
}

/// Ast validators
pub mod validators {
    use crate::AstVisitor;
    use ddlog_utils::bvec;

    pub use super::{generic_arg_count::GenericArgCount, struct_field_count::StructFieldCount};

    /// Returns a vec of the default ast validators
    #[inline]
    pub fn default() -> Vec<Box<dyn AstVisitor>> {
        bvec![StructFieldCount::default(), GenericArgCount::default()]
    }
}

#[cfg(test)]
mod tests {
    use crate::{visitor, AstVisitor, NodeCache, RuleCtx};
    use ddlog_diagnostics::{Diagnostic, DiagnosticConfig, FileCache, FileId, Interner, Rope};

    #[track_caller]
    pub(super) fn validate_fail<V: AstVisitor>(source: &str, test_name: &str, validator: V) {
        test_validate(source, test_name, validator, false)
    }

    #[track_caller]
    pub(super) fn validate_pass<V: AstVisitor>(source: &str, test_name: &str, validator: V) {
        test_validate(source, test_name, validator, true)
    }

    #[track_caller]
    pub(super) fn test_validate<V: AstVisitor>(
        source: &str,
        test_name: &str,
        mut validator: V,
        should_pass: bool,
    ) {
        let pass_dir = if should_pass { "pass" } else { "fail" };
        assert!(
            !test_name.is_empty()
                && test_name.chars().next().unwrap().is_alphabetic()
                && test_name
                    .chars()
                    .all(|char| char.is_alphanumeric() || matches!(char, '-' | '_')),
            "test name {test_name:?} contains invalid characters, \
            test names should be in the [a-zA-Z][a-zA-Z0-9_-]+ format",
        );

        let interner = Interner::new();
        let file =
            FileId::new(interner.get_or_intern(&format!("tests/{pass_dir}/{test_name}.ddlog")));

        let mut file_cache = FileCache::new(interner.clone());
        file_cache.add_str(file, source);

        let cache = NodeCache::from_interner(interner.clone());
        let (parsed, _) = crate::parse(file, source, cache);

        let mut ctx = RuleCtx::new(file, Rope::from_str(source), interner);
        visitor::apply_visitor(parsed.syntax(), &mut validator, &mut ctx);

        let diagnostics = ctx.into_diagnostics();
        if should_pass {
            assert!(
                diagnostics.is_empty(),
                "the diagnostics for {test_name} were not empty",
            );
        }

        let output = format_output(diagnostics, source, &mut file_cache);
        expect_test::expect_file![format!("./tests/{test_name}.expected")].assert_eq(&output);
    }

    fn format_output(
        diagnostics: Vec<Diagnostic>,
        source: &str,
        file_cache: &mut FileCache,
    ) -> String {
        let (diagnostic_config, mut output, mut buffer) = (
            DiagnosticConfig::new().with_color(false),
            String::new(),
            Vec::new(),
        );

        for diagnostic in diagnostics {
            diagnostic
                .emit_to(&diagnostic_config, file_cache, &mut buffer)
                .expect("failed to emit diagnostic");

            output.push_str(
                std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer"),
            );
            buffer.clear();
        }
        output.push_str("-----\n");
        output.push_str(source);
        output
    }
}
