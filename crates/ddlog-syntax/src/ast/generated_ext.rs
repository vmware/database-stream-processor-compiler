use crate::ast::{
    nodes::{
        Attribute, BracketedStructField, ConstDef, EnumDef, EnumVariant, FunctionDef, RelationDef,
        StructDef, StructFields, TupleType, VarRef,
    },
    tokens::Ident,
    AstNode, AstToken,
};
use cstree::TextRange;
use ddlog_diagnostics::{IStr, Interner};

impl Ident {
    #[inline]
    pub fn interned(&self) -> IStr {
        IStr::new(self.syntax().green().text_key())
    }
}

impl VarRef {
    pub fn interned(&self) -> Option<IStr> {
        self.ident().map(|ident| ident.interned())
    }
}

impl RelationDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name().map(|ident| ident.interned())
    }
}

impl FunctionDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name().map(|ident| ident.interned())
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
            .or(if include_keyword { keyword } else { None })
            .unwrap_or_else(|| self.trimmed_range());

        start.cover(end)
    }

    /// Returns `true` if the enum has a `#[deprecated]` attribute
    pub fn is_deprecated(&self, interner: &Interner) -> bool {
        self.attributes().any(|attr| attr.is_deprecated(interner))
    }
}

impl EnumDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name().map(|ident| ident.interned())
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
        self.name().map(|ident| ident.interned())
    }

    pub fn signature_span(&self) -> TextRange {
        let keyword = self.keyword().map(|function| function.text_range());
        let name = self.name().as_ref().map(|name| name.text_range());

        let start = keyword.or(name).unwrap_or_else(|| self.trimmed_range());
        let end = name.or(keyword).unwrap_or_else(|| self.trimmed_range());
        start.cover(end)
    }

    pub fn total_fields(&self) -> usize {
        self.fields().map_or(0, |fields| fields.len())
    }

    pub fn is_unit_struct(&self) -> bool {
        self.fields().is_some()
    }

    pub fn is_bracketed_struct(&self) -> bool {
        self.fields()
            .map_or(false, |fields| fields.is_bracketed_struct_fields())
    }

    pub fn is_tuple_struct(&self) -> bool {
        self.fields()
            .map_or(false, |fields| fields.is_tuple_struct_fields())
    }
}

impl ConstDef {
    pub fn ident(&self) -> Option<IStr> {
        self.name().map(|ident| ident.interned())
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

impl TupleType {
    pub fn is_empty(&self) -> bool {
        self.elements().len() == 0
    }
}

impl StructFields {
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::BracketedStructFields(fields) => fields.fields().count(),
            Self::TupleStructFields(fields) => fields.fields().count(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl BracketedStructField {
    pub fn ident(&self) -> Option<IStr> {
        self.name().map(|ident| ident.interned())
    }
}
