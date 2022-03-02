use crate::ast::{
    nodes::{Attribute, ConstDef, EnumDef, EnumVariant, FunctionDef, StructDef},
    AstNode, AstToken,
};
use cstree::TextRange;
use ddlog_diagnostics::{IStr, Interner};

impl FunctionDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name()
            .map(|ident| IStr::new(ident.syntax().green().text_key()))
    }

    /// Get the span of a function's signature
    ///
    /// If `include_keyword` is set then the `function` keyword will be
    /// included in the signature's span
    pub fn signature_span(&self, include_keyword: bool) -> TextRange {
        let keyword = self.keyword().map(|function| function.text_range());
        let name = self.name().as_ref().map(|name| name.text_range());
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

    /// Returns `true` if the enum has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl EnumDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name()
            .map(|ident| IStr::new(ident.syntax().green().text_key()))
    }

    /// Returns `true` if the enum has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl EnumVariant {
    /// Returns `true` if the variant has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl StructDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name()
            .map(|ident| IStr::new(ident.syntax().green().text_key()))
    }
}

impl ConstDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name()
            .map(|ident| IStr::new(ident.syntax().green().text_key()))
    }
}

impl Attribute {
    /// Returns `true` if any of the underlying attributes are `deprecated`
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        for pair in self.attr_pairs() {
            if let Some(name) = pair.name() {
                if name.lexical_eq("deprecated", interner) {
                    return true;
                }
            }
        }

        false
    }
}
