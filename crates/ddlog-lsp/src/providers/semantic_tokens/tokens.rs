use lspower::lsp::{SemanticTokenModifier, SemanticTokenType};
use std::ops::BitOrAssign;

#[derive(Default)]
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

define_semantic_token_types![
    (ANGLE, "angle"),
    (ARITHMETIC, "arithmetic"),
    (ATTRIBUTE, "attribute"),
    (BITWISE, "bitwise"),
    (BOOLEAN, "boolean"),
    (BRACE, "brace"),
    (BRACKET, "bracket"),
    (BUILTIN_ATTRIBUTE, "builtinAttribute"),
    (BUILTIN_TYPE, "builtinType"),
    (CHAR, "character"),
    (COLON, "colon"),
    (COMMA, "comma"),
    (COMPARISON, "comparison"),
    (CONST_PARAMETER, "constParameter"),
    (DOT, "dot"),
    (ESCAPE_SEQUENCE, "escapeSequence"),
    (FORMAT_SPECIFIER, "formatSpecifier"),
    (GENERIC, "generic"),
    (LABEL, "label"),
    (LIFETIME, "lifetime"),
    (LOGICAL, "logical"),
    (OPERATOR, "operator"),
    (PARENTHESIS, "parenthesis"),
    (PUNCTUATION, "punctuation"),
    (SELF_KEYWORD, "selfKeyword"),
    (SEMICOLON, "semicolon"),
    (TYPE_ALIAS, "typeAlias"),
    (UNION, "union"),
    (UNRESOLVED_REFERENCE, "unresolvedReference"),
];

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

define_semantic_token_modifiers![
    (CONSTANT, "constant"),
    (CONTROL_FLOW, "controlFlow"),
    (INJECTED, "injected"),
    (MUTABLE, "mutable"),
    (CONSUMING, "consuming"),
    (ASYNC, "async"),
    (LIBRARY, "library"),
    (PUBLIC, "public"),
    (UNSAFE, "unsafe"),
    (ATTRIBUTE_MODIFIER, "attribute"),
    (TRAIT_MODIFIER, "trait"),
    (CALLABLE, "callable"),
    (INTRA_DOC_LINK, "intraDocLink"),
    (REFERENCE, "reference"),
];
