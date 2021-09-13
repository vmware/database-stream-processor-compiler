use crate::ast::{
    nodes::{Attributes, FunctionDef},
    AstNode, AstToken,
};
use cstree::TextRange;
use ddlog_diagnostics::Interner;

impl FunctionDef {
    /// Get the span of a function's signature
    ///
    /// If `include_keyword` is set then the `function` keyword will be
    /// included in the signature's span
    pub fn signature_span(&self, include_keyword: bool) -> TextRange {
        let keyword = self.keyword().map(|function| function.text_range());
        let name = self
            .name()
            .as_ref()
            .and_then(|name| name.ident())
            .map(|name| name.text_range());
        let generics = self.generics().map(|generics| generics.trimmed_range());
        let args = self.args().map(|args| args.trimmed_range());
        let ret = self.ret().map(|ret| ret.trimmed_range());

        let start = if include_keyword {
            keyword.or(name)
        } else {
            name
        }
        .or(generics)
        .or(args)
        .or(ret)
        .unwrap_or_else(|| self.trimmed_range());

        let end = ret
            .or(args)
            .or(generics)
            .or(name)
            .or_else(|| if include_keyword { keyword } else { None })
            .unwrap_or_else(|| self.trimmed_range());

        start.intersect(end).unwrap_or_else(|| self.trimmed_range())
    }
}

impl Attributes {
    /// Returns `true` if any of the underlying attributes are `deprecated`
    pub fn any_are_deprecated(&self, interner: &Interner) -> bool {
        for attribute in self.attributes() {
            for pair in attribute.attr_pairs() {
                if let Some(ident) = pair.ident() {
                    if ident.lexical_eq("deprecated", interner) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
