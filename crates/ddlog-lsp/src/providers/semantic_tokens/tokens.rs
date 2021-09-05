use lspower::lsp::{SemanticTokenModifier, SemanticTokenType};
use std::ops::BitOrAssign;

// TODO: Better debug impl
#[derive(Debug, Default)]
#[repr(transparent)]
pub(crate) struct ModifierSet(pub(crate) u32);

impl ModifierSet {
    pub const fn empty() -> Self {
        Self(0)
    }
}

impl BitOrAssign<SemanticTokenModifier> for ModifierSet {
    #[track_caller]
    fn bitor_assign(&mut self, rhs: SemanticTokenModifier) {
        let idx = SUPPORTED_MODIFIERS
            .iter()
            .position(|modifier| modifier == &rhs)
            .unwrap_or_else(|| panic!("unrecognized semantic token modifier: {:?}", rhs));

        self.0 |= 1 << idx;
    }
}

macro_rules! define_semantic_token_types {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub(crate) const $ident: SemanticTokenType = SemanticTokenType::new($string);)*

        pub(crate) const SUPPORTED_TYPES: &[SemanticTokenType] = &[
            SemanticTokenType::COMMENT,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::REGEXP,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::TYPE,
            SemanticTokenType::STRUCT,
            SemanticTokenType::CLASS,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::ENUM,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::METHOD,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::MACRO,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PARAMETER,
            $($ident),*
        ];
    };
}

define_semantic_token_types! {}

macro_rules! define_semantic_token_modifiers {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub(crate) const $ident: SemanticTokenModifier = SemanticTokenModifier::new($string);)*

        pub(crate) const SUPPORTED_MODIFIERS: &[SemanticTokenModifier] = &[
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::READONLY,
            $($ident),*
        ];
    };
}

define_semantic_token_modifiers! {}
