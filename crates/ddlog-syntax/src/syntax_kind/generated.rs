#[derive(logos :: Logos)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    AMPERSAND = 0u16,
    AMPERSAND_EQ = 1u16,
    AND = 2u16,
    ARRAY_ACCESS = 3u16,
    ARRAY_EXPR_ELEM = 4u16,
    ARRAY_INIT_EXPR = 5u16,
    ASSIGN = 6u16,
    ASSIGN_OP = 7u16,
    AS_KW = 8u16,
    ATTRIBUTE = 9u16,
    ATTR_NAME = 10u16,
    ATTR_PAIR = 11u16,
    BANG = 12u16,
    BIN_EXPR = 13u16,
    BIN_OP = 14u16,
    BLOCK = 15u16,
    BOOL = 16u16,
    BRACKETED_STRUCT_FIELD = 17u16,
    BRACKETED_STRUCT_FIELDS = 18u16,
    BREAK_EXPR = 19u16,
    BREAK_KW = 20u16,
    CARET = 21u16,
    CARET_EQ = 22u16,
    CLOSURE_ARG = 23u16,
    CLOSURE_EXPR = 24u16,
    COLON = 25u16,
    COMMA = 26u16,
    #[regex("//.*")]
    #[regex("///.*")]
    #[token("/*", lex_block_comment)]
    COMMENT = 27u16,
    CONST_DEF = 28u16,
    CONST_KW = 29u16,
    CONST_NAME = 30u16,
    CONST_VALUE = 31u16,
    CONTINUE_EXPR = 32u16,
    CONTINUE_KW = 33u16,
    DOT = 34u16,
    DOUBLE_COLON = 35u16,
    ELSE_BLOCK = 36u16,
    ELSE_KW = 37u16,
    ENUM_DEF = 38u16,
    ENUM_KW = 39u16,
    ENUM_NAME = 40u16,
    ENUM_VARIANT = 41u16,
    ENUM_VARIANTS = 42u16,
    ENUM_VARIANT_BODY = 43u16,
    ENUM_VARIANT_NAME = 44u16,
    EOF = 45u16,
    EQ = 46u16,
    EQEQ = 47u16,
    #[error]
    ERROR = 48u16,
    EXPR = 49u16,
    EXPR_STMT = 50u16,
    FALSE_KW = 51u16,
    FIELD_ACCESS = 52u16,
    FIELD_ACCESSOR = 53u16,
    FIELD_ACCESSOR_NAME = 54u16,
    FN_KW = 55u16,
    FOR_EXPR = 56u16,
    FOR_KW = 57u16,
    FUNCTION_ARG = 58u16,
    FUNCTION_ARGS = 59u16,
    FUNCTION_CALL = 60u16,
    FUNCTION_CALL_ARG = 61u16,
    FUNCTION_DEF = 62u16,
    FUNCTION_NAME = 63u16,
    FUNCTION_RETURN = 64u16,
    FUNCTION_RETURN_TYPE = 65u16,
    FUNCTION_TYPE = 66u16,
    FUNCTION_TYPE_ARG = 67u16,
    FUNCTION_TYPE_ARGS = 68u16,
    GENERICS = 69u16,
    GENERIC_ARG = 70u16,
    GENERIC_TYPE = 71u16,
    HASH_BRACK = 72u16,
    #[regex("[A-Za-z_'][A-Za-z0-9_']*")]
    IDENT = 73u16,
    IF_BLOCK = 74u16,
    IF_EXPR = 75u16,
    IF_KW = 76u16,
    IMPL_BLOCK = 77u16,
    IMPL_BLOCK_CONTENTS = 78u16,
    IMPL_KW = 79u16,
    IN_KW = 80u16,
    ITEM = 81u16,
    LET_KW = 82u16,
    LITERAL = 83u16,
    LOOP_EXPR = 84u16,
    LOOP_KW = 85u16,
    L_ANGLE = 86u16,
    L_ANGLE_EQ = 87u16,
    L_BRACK = 88u16,
    L_CURLY = 89u16,
    L_PAREN = 90u16,
    MATCH_ARM = 91u16,
    MATCH_EXPR = 92u16,
    MATCH_KW = 93u16,
    MINUS = 94u16,
    MINUS_EQ = 95u16,
    MODIFIER = 96u16,
    NEQ = 97u16,
    NUMBER = 98u16,
    #[regex("[0-9][0-9_]*")]
    #[regex("0b[0-1][0-1_]*")]
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*")]
    NUMBER_LITERAL = 99u16,
    OR = 100u16,
    PAREN_EXPR = 101u16,
    PATH = 102u16,
    PATH_SEGMENT = 103u16,
    PATH_TAIL = 104u16,
    PATTERN = 105u16,
    PERCENT = 106u16,
    PERCENT_EQ = 107u16,
    PIPE = 108u16,
    PIPE_EQ = 109u16,
    PLUS = 110u16,
    PLUS_EQ = 111u16,
    PUB_KW = 112u16,
    QUALIFIED_REF = 113u16,
    RETURN_KW = 114u16,
    RET_EXPR = 115u16,
    RIGHT_ARROW = 116u16,
    RIGHT_ROCKET = 117u16,
    ROOT = 118u16,
    R_ANGLE = 119u16,
    R_ANGLE_EQ = 120u16,
    R_BRACK = 121u16,
    R_CURLY = 122u16,
    R_PAREN = 123u16,
    SEMICOLON = 124u16,
    SHL = 125u16,
    SHL_EQ = 126u16,
    SHR = 127u16,
    SHR_EQ = 128u16,
    SLASH = 129u16,
    SLASH_EQ = 130u16,
    STAR = 131u16,
    STAR_EQ = 132u16,
    STMT = 133u16,
    STRING = 134u16,
    STRING_LITERAL = 135u16,
    STRUCT_DEF = 136u16,
    STRUCT_FIELDS = 137u16,
    STRUCT_FIELD_NAME = 138u16,
    STRUCT_INIT_EXPR = 139u16,
    STRUCT_INIT_FIELD = 140u16,
    STRUCT_KW = 141u16,
    STRUCT_NAME = 142u16,
    STRUCT_PATTERN = 143u16,
    STRUCT_PATTERN_FIELD = 144u16,
    STRUCT_PATTERN_FIELD_NAME = 145u16,
    TOMBSTONE = 146u16,
    TRUE_KW = 147u16,
    TUPLE_EXPR_ELEM = 148u16,
    TUPLE_INIT_EXPR = 149u16,
    TUPLE_PATTERN = 150u16,
    TUPLE_PATTERN_ELEM = 151u16,
    TUPLE_STRUCT_FIELD = 152u16,
    TUPLE_STRUCT_FIELDS = 153u16,
    TUPLE_TYPE = 154u16,
    TUPLE_TYPE_ELEM = 155u16,
    TYPE = 156u16,
    TYPE_ALIAS = 157u16,
    TYPE_KW = 158u16,
    UNARY_EXPR = 159u16,
    UNARY_OP = 160u16,
    USE_ALIAS = 161u16,
    USE_ALIAS_NAME = 162u16,
    USE_BRANCH = 163u16,
    USE_BRANCH_OR_ALIAS = 164u16,
    USE_DEF = 165u16,
    USE_KW = 166u16,
    USE_TREE = 167u16,
    VARIANT_STRUCT = 168u16,
    VARIANT_STRUCT_FIELD = 169u16,
    VARIANT_STRUCT_FIELD_NAME = 170u16,
    VARIANT_TUPLE = 171u16,
    VARIANT_TUPLE_ELEM = 172u16,
    VAR_DECL = 173u16,
    VAR_REF = 174u16,
    WHILE_EXPR = 175u16,
    WHILE_KW = 176u16,
    #[regex("[\n\t\r ]+")]
    WHITESPACE = 177u16,
}
impl SyntaxKind {
    /// The maximum discriminant of the [`SyntaxKind`] enum
    pub const MAXIMUM_DISCRIMINANT: u16 = 177u16;
}
fn lex_block_comment(lexer: &mut logos::Lexer<'_, SyntaxKind>) -> bool {
    let remainder = lexer.remainder();
    let mut nesting = 0;
    for (idx, _) in remainder.char_indices() {
        match remainder.get(idx..idx + 2) {
            Some("*/") if nesting == 0 => {
                lexer.bump(idx + 2);
                return true;
            }
            Some("*/") => nesting -= 1,
            Some("/*") => nesting += 1,
            Some(_) => continue,
            None => break,
        }
    }
    false
}
impl ::core::fmt::Debug for SyntaxKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::AMPERSAND => ::core::fmt::Formatter::write_str(f, "AMPERSAND"),
            Self::AMPERSAND_EQ => ::core::fmt::Formatter::write_str(f, "AMPERSAND_EQ"),
            Self::AND => ::core::fmt::Formatter::write_str(f, "AND"),
            Self::ARRAY_ACCESS => ::core::fmt::Formatter::write_str(f, "ARRAY_ACCESS"),
            Self::ARRAY_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "ARRAY_EXPR_ELEM"),
            Self::ARRAY_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "ARRAY_INIT_EXPR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ASSIGN_OP => ::core::fmt::Formatter::write_str(f, "ASSIGN_OP"),
            Self::AS_KW => ::core::fmt::Formatter::write_str(f, "AS_KW"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
            Self::ATTR_NAME => ::core::fmt::Formatter::write_str(f, "ATTR_NAME"),
            Self::ATTR_PAIR => ::core::fmt::Formatter::write_str(f, "ATTR_PAIR"),
            Self::BANG => ::core::fmt::Formatter::write_str(f, "BANG"),
            Self::BIN_EXPR => ::core::fmt::Formatter::write_str(f, "BIN_EXPR"),
            Self::BIN_OP => ::core::fmt::Formatter::write_str(f, "BIN_OP"),
            Self::BLOCK => ::core::fmt::Formatter::write_str(f, "BLOCK"),
            Self::BOOL => ::core::fmt::Formatter::write_str(f, "BOOL"),
            Self::BRACKETED_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "BRACKETED_STRUCT_FIELD")
            }
            Self::BRACKETED_STRUCT_FIELDS => {
                ::core::fmt::Formatter::write_str(f, "BRACKETED_STRUCT_FIELDS")
            }
            Self::BREAK_EXPR => ::core::fmt::Formatter::write_str(f, "BREAK_EXPR"),
            Self::BREAK_KW => ::core::fmt::Formatter::write_str(f, "BREAK_KW"),
            Self::CARET => ::core::fmt::Formatter::write_str(f, "CARET"),
            Self::CARET_EQ => ::core::fmt::Formatter::write_str(f, "CARET_EQ"),
            Self::CLOSURE_ARG => ::core::fmt::Formatter::write_str(f, "CLOSURE_ARG"),
            Self::CLOSURE_EXPR => ::core::fmt::Formatter::write_str(f, "CLOSURE_EXPR"),
            Self::COLON => ::core::fmt::Formatter::write_str(f, "COLON"),
            Self::COMMA => ::core::fmt::Formatter::write_str(f, "COMMA"),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::CONST_DEF => ::core::fmt::Formatter::write_str(f, "CONST_DEF"),
            Self::CONST_KW => ::core::fmt::Formatter::write_str(f, "CONST_KW"),
            Self::CONST_NAME => ::core::fmt::Formatter::write_str(f, "CONST_NAME"),
            Self::CONST_VALUE => ::core::fmt::Formatter::write_str(f, "CONST_VALUE"),
            Self::CONTINUE_EXPR => ::core::fmt::Formatter::write_str(f, "CONTINUE_EXPR"),
            Self::CONTINUE_KW => ::core::fmt::Formatter::write_str(f, "CONTINUE_KW"),
            Self::DOT => ::core::fmt::Formatter::write_str(f, "DOT"),
            Self::DOUBLE_COLON => ::core::fmt::Formatter::write_str(f, "DOUBLE_COLON"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::ELSE_KW => ::core::fmt::Formatter::write_str(f, "ELSE_KW"),
            Self::ENUM_DEF => ::core::fmt::Formatter::write_str(f, "ENUM_DEF"),
            Self::ENUM_KW => ::core::fmt::Formatter::write_str(f, "ENUM_KW"),
            Self::ENUM_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_NAME"),
            Self::ENUM_VARIANT => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT"),
            Self::ENUM_VARIANTS => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANTS"),
            Self::ENUM_VARIANT_BODY => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_BODY"),
            Self::ENUM_VARIANT_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_NAME"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EQ => ::core::fmt::Formatter::write_str(f, "EQ"),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "EQEQ"),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "ERROR"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FALSE_KW => ::core::fmt::Formatter::write_str(f, "FALSE_KW"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR"),
            Self::FIELD_ACCESSOR_NAME => {
                ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR_NAME")
            }
            Self::FN_KW => ::core::fmt::Formatter::write_str(f, "FN_KW"),
            Self::FOR_EXPR => ::core::fmt::Formatter::write_str(f, "FOR_EXPR"),
            Self::FOR_KW => ::core::fmt::Formatter::write_str(f, "FOR_KW"),
            Self::FUNCTION_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARG"),
            Self::FUNCTION_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARGS"),
            Self::FUNCTION_CALL => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL"),
            Self::FUNCTION_CALL_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL_ARG"),
            Self::FUNCTION_DEF => ::core::fmt::Formatter::write_str(f, "FUNCTION_DEF"),
            Self::FUNCTION_NAME => ::core::fmt::Formatter::write_str(f, "FUNCTION_NAME"),
            Self::FUNCTION_RETURN => ::core::fmt::Formatter::write_str(f, "FUNCTION_RETURN"),
            Self::FUNCTION_RETURN_TYPE => {
                ::core::fmt::Formatter::write_str(f, "FUNCTION_RETURN_TYPE")
            }
            Self::FUNCTION_TYPE => ::core::fmt::Formatter::write_str(f, "FUNCTION_TYPE"),
            Self::FUNCTION_TYPE_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_TYPE_ARG"),
            Self::FUNCTION_TYPE_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_TYPE_ARGS"),
            Self::GENERICS => ::core::fmt::Formatter::write_str(f, "GENERICS"),
            Self::GENERIC_ARG => ::core::fmt::Formatter::write_str(f, "GENERIC_ARG"),
            Self::GENERIC_TYPE => ::core::fmt::Formatter::write_str(f, "GENERIC_TYPE"),
            Self::HASH_BRACK => ::core::fmt::Formatter::write_str(f, "HASH_BRACK"),
            Self::IDENT => ::core::fmt::Formatter::write_str(f, "IDENT"),
            Self::IF_BLOCK => ::core::fmt::Formatter::write_str(f, "IF_BLOCK"),
            Self::IF_EXPR => ::core::fmt::Formatter::write_str(f, "IF_EXPR"),
            Self::IF_KW => ::core::fmt::Formatter::write_str(f, "IF_KW"),
            Self::IMPL_BLOCK => ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK"),
            Self::IMPL_BLOCK_CONTENTS => {
                ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK_CONTENTS")
            }
            Self::IMPL_KW => ::core::fmt::Formatter::write_str(f, "IMPL_KW"),
            Self::IN_KW => ::core::fmt::Formatter::write_str(f, "IN_KW"),
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LET_KW => ::core::fmt::Formatter::write_str(f, "LET_KW"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
            Self::LOOP_EXPR => ::core::fmt::Formatter::write_str(f, "LOOP_EXPR"),
            Self::LOOP_KW => ::core::fmt::Formatter::write_str(f, "LOOP_KW"),
            Self::L_ANGLE => ::core::fmt::Formatter::write_str(f, "L_ANGLE"),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "L_ANGLE_EQ"),
            Self::L_BRACK => ::core::fmt::Formatter::write_str(f, "L_BRACK"),
            Self::L_CURLY => ::core::fmt::Formatter::write_str(f, "L_CURLY"),
            Self::L_PAREN => ::core::fmt::Formatter::write_str(f, "L_PAREN"),
            Self::MATCH_ARM => ::core::fmt::Formatter::write_str(f, "MATCH_ARM"),
            Self::MATCH_EXPR => ::core::fmt::Formatter::write_str(f, "MATCH_EXPR"),
            Self::MATCH_KW => ::core::fmt::Formatter::write_str(f, "MATCH_KW"),
            Self::MINUS => ::core::fmt::Formatter::write_str(f, "MINUS"),
            Self::MINUS_EQ => ::core::fmt::Formatter::write_str(f, "MINUS_EQ"),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::NEQ => ::core::fmt::Formatter::write_str(f, "NEQ"),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::NUMBER_LITERAL => ::core::fmt::Formatter::write_str(f, "NUMBER_LITERAL"),
            Self::OR => ::core::fmt::Formatter::write_str(f, "OR"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATH => ::core::fmt::Formatter::write_str(f, "PATH"),
            Self::PATH_SEGMENT => ::core::fmt::Formatter::write_str(f, "PATH_SEGMENT"),
            Self::PATH_TAIL => ::core::fmt::Formatter::write_str(f, "PATH_TAIL"),
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
            Self::PERCENT => ::core::fmt::Formatter::write_str(f, "PERCENT"),
            Self::PERCENT_EQ => ::core::fmt::Formatter::write_str(f, "PERCENT_EQ"),
            Self::PIPE => ::core::fmt::Formatter::write_str(f, "PIPE"),
            Self::PIPE_EQ => ::core::fmt::Formatter::write_str(f, "PIPE_EQ"),
            Self::PLUS => ::core::fmt::Formatter::write_str(f, "PLUS"),
            Self::PLUS_EQ => ::core::fmt::Formatter::write_str(f, "PLUS_EQ"),
            Self::PUB_KW => ::core::fmt::Formatter::write_str(f, "PUB_KW"),
            Self::QUALIFIED_REF => ::core::fmt::Formatter::write_str(f, "QUALIFIED_REF"),
            Self::RETURN_KW => ::core::fmt::Formatter::write_str(f, "RETURN_KW"),
            Self::RET_EXPR => ::core::fmt::Formatter::write_str(f, "RET_EXPR"),
            Self::RIGHT_ARROW => ::core::fmt::Formatter::write_str(f, "RIGHT_ARROW"),
            Self::RIGHT_ROCKET => ::core::fmt::Formatter::write_str(f, "RIGHT_ROCKET"),
            Self::ROOT => ::core::fmt::Formatter::write_str(f, "ROOT"),
            Self::R_ANGLE => ::core::fmt::Formatter::write_str(f, "R_ANGLE"),
            Self::R_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "R_ANGLE_EQ"),
            Self::R_BRACK => ::core::fmt::Formatter::write_str(f, "R_BRACK"),
            Self::R_CURLY => ::core::fmt::Formatter::write_str(f, "R_CURLY"),
            Self::R_PAREN => ::core::fmt::Formatter::write_str(f, "R_PAREN"),
            Self::SEMICOLON => ::core::fmt::Formatter::write_str(f, "SEMICOLON"),
            Self::SHL => ::core::fmt::Formatter::write_str(f, "SHL"),
            Self::SHL_EQ => ::core::fmt::Formatter::write_str(f, "SHL_EQ"),
            Self::SHR => ::core::fmt::Formatter::write_str(f, "SHR"),
            Self::SHR_EQ => ::core::fmt::Formatter::write_str(f, "SHR_EQ"),
            Self::SLASH => ::core::fmt::Formatter::write_str(f, "SLASH"),
            Self::SLASH_EQ => ::core::fmt::Formatter::write_str(f, "SLASH_EQ"),
            Self::STAR => ::core::fmt::Formatter::write_str(f, "STAR"),
            Self::STAR_EQ => ::core::fmt::Formatter::write_str(f, "STAR_EQ"),
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRING_LITERAL => ::core::fmt::Formatter::write_str(f, "STRING_LITERAL"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_FIELDS => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELDS"),
            Self::STRUCT_FIELD_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELD_NAME"),
            Self::STRUCT_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_EXPR"),
            Self::STRUCT_INIT_FIELD => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_FIELD"),
            Self::STRUCT_KW => ::core::fmt::Formatter::write_str(f, "STRUCT_KW"),
            Self::STRUCT_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_NAME"),
            Self::STRUCT_PATTERN => ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN"),
            Self::STRUCT_PATTERN_FIELD => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD")
            }
            Self::STRUCT_PATTERN_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD_NAME")
            }
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE_KW => ::core::fmt::Formatter::write_str(f, "TRUE_KW"),
            Self::TUPLE_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_EXPR_ELEM"),
            Self::TUPLE_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "TUPLE_INIT_EXPR"),
            Self::TUPLE_PATTERN => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN"),
            Self::TUPLE_PATTERN_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN_ELEM"),
            Self::TUPLE_STRUCT_FIELD => ::core::fmt::Formatter::write_str(f, "TUPLE_STRUCT_FIELD"),
            Self::TUPLE_STRUCT_FIELDS => {
                ::core::fmt::Formatter::write_str(f, "TUPLE_STRUCT_FIELDS")
            }
            Self::TUPLE_TYPE => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE"),
            Self::TUPLE_TYPE_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE_ELEM"),
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::TYPE_ALIAS => ::core::fmt::Formatter::write_str(f, "TYPE_ALIAS"),
            Self::TYPE_KW => ::core::fmt::Formatter::write_str(f, "TYPE_KW"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_ALIAS_NAME => ::core::fmt::Formatter::write_str(f, "USE_ALIAS_NAME"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_BRANCH_OR_ALIAS => {
                ::core::fmt::Formatter::write_str(f, "USE_BRANCH_OR_ALIAS")
            }
            Self::USE_DEF => ::core::fmt::Formatter::write_str(f, "USE_DEF"),
            Self::USE_KW => ::core::fmt::Formatter::write_str(f, "USE_KW"),
            Self::USE_TREE => ::core::fmt::Formatter::write_str(f, "USE_TREE"),
            Self::VARIANT_STRUCT => ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT"),
            Self::VARIANT_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD")
            }
            Self::VARIANT_STRUCT_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD_NAME")
            }
            Self::VARIANT_TUPLE => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE"),
            Self::VARIANT_TUPLE_ELEM => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE_ELEM"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::WHILE_EXPR => ::core::fmt::Formatter::write_str(f, "WHILE_EXPR"),
            Self::WHILE_KW => ::core::fmt::Formatter::write_str(f, "WHILE_KW"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
        }
    }
}
impl ::core::fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::AMPERSAND => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '&'),
            Self::AMPERSAND_EQ => ::core::fmt::Formatter::write_str(f, "&="),
            Self::AND => ::core::fmt::Formatter::write_str(f, "AND"),
            Self::ARRAY_ACCESS => ::core::fmt::Formatter::write_str(f, "ARRAY_ACCESS"),
            Self::ARRAY_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "ARRAY_EXPR_ELEM"),
            Self::ARRAY_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "ARRAY_INIT_EXPR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ASSIGN_OP => ::core::fmt::Formatter::write_str(f, "ASSIGN_OP"),
            Self::AS_KW => ::core::fmt::Formatter::write_str(f, "as"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
            Self::ATTR_NAME => ::core::fmt::Formatter::write_str(f, "ATTR_NAME"),
            Self::ATTR_PAIR => ::core::fmt::Formatter::write_str(f, "ATTR_PAIR"),
            Self::BANG => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '!'),
            Self::BIN_EXPR => ::core::fmt::Formatter::write_str(f, "BIN_EXPR"),
            Self::BIN_OP => ::core::fmt::Formatter::write_str(f, "BIN_OP"),
            Self::BLOCK => ::core::fmt::Formatter::write_str(f, "BLOCK"),
            Self::BOOL => ::core::fmt::Formatter::write_str(f, "BOOL"),
            Self::BRACKETED_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "BRACKETED_STRUCT_FIELD")
            }
            Self::BRACKETED_STRUCT_FIELDS => {
                ::core::fmt::Formatter::write_str(f, "BRACKETED_STRUCT_FIELDS")
            }
            Self::BREAK_EXPR => ::core::fmt::Formatter::write_str(f, "BREAK_EXPR"),
            Self::BREAK_KW => ::core::fmt::Formatter::write_str(f, "break"),
            Self::CARET => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '^'),
            Self::CARET_EQ => ::core::fmt::Formatter::write_str(f, "^="),
            Self::CLOSURE_ARG => ::core::fmt::Formatter::write_str(f, "CLOSURE_ARG"),
            Self::CLOSURE_EXPR => ::core::fmt::Formatter::write_str(f, "CLOSURE_EXPR"),
            Self::COLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ':'),
            Self::COMMA => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ','),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::CONST_DEF => ::core::fmt::Formatter::write_str(f, "CONST_DEF"),
            Self::CONST_KW => ::core::fmt::Formatter::write_str(f, "const"),
            Self::CONST_NAME => ::core::fmt::Formatter::write_str(f, "CONST_NAME"),
            Self::CONST_VALUE => ::core::fmt::Formatter::write_str(f, "CONST_VALUE"),
            Self::CONTINUE_EXPR => ::core::fmt::Formatter::write_str(f, "CONTINUE_EXPR"),
            Self::CONTINUE_KW => ::core::fmt::Formatter::write_str(f, "continue"),
            Self::DOT => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '.'),
            Self::DOUBLE_COLON => ::core::fmt::Formatter::write_str(f, "::"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::ELSE_KW => ::core::fmt::Formatter::write_str(f, "else"),
            Self::ENUM_DEF => ::core::fmt::Formatter::write_str(f, "ENUM_DEF"),
            Self::ENUM_KW => ::core::fmt::Formatter::write_str(f, "enum"),
            Self::ENUM_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_NAME"),
            Self::ENUM_VARIANT => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT"),
            Self::ENUM_VARIANTS => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANTS"),
            Self::ENUM_VARIANT_BODY => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_BODY"),
            Self::ENUM_VARIANT_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_NAME"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EQ => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '='),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "=="),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "???"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FALSE_KW => ::core::fmt::Formatter::write_str(f, "false"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR"),
            Self::FIELD_ACCESSOR_NAME => {
                ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR_NAME")
            }
            Self::FN_KW => ::core::fmt::Formatter::write_str(f, "fn"),
            Self::FOR_EXPR => ::core::fmt::Formatter::write_str(f, "FOR_EXPR"),
            Self::FOR_KW => ::core::fmt::Formatter::write_str(f, "for"),
            Self::FUNCTION_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARG"),
            Self::FUNCTION_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARGS"),
            Self::FUNCTION_CALL => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL"),
            Self::FUNCTION_CALL_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL_ARG"),
            Self::FUNCTION_DEF => ::core::fmt::Formatter::write_str(f, "FUNCTION_DEF"),
            Self::FUNCTION_NAME => ::core::fmt::Formatter::write_str(f, "FUNCTION_NAME"),
            Self::FUNCTION_RETURN => ::core::fmt::Formatter::write_str(f, "FUNCTION_RETURN"),
            Self::FUNCTION_RETURN_TYPE => {
                ::core::fmt::Formatter::write_str(f, "FUNCTION_RETURN_TYPE")
            }
            Self::FUNCTION_TYPE => ::core::fmt::Formatter::write_str(f, "FUNCTION_TYPE"),
            Self::FUNCTION_TYPE_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_TYPE_ARG"),
            Self::FUNCTION_TYPE_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_TYPE_ARGS"),
            Self::GENERICS => ::core::fmt::Formatter::write_str(f, "GENERICS"),
            Self::GENERIC_ARG => ::core::fmt::Formatter::write_str(f, "GENERIC_ARG"),
            Self::GENERIC_TYPE => ::core::fmt::Formatter::write_str(f, "GENERIC_TYPE"),
            Self::HASH_BRACK => ::core::fmt::Formatter::write_str(f, "#["),
            Self::IDENT => ::core::fmt::Formatter::write_str(f, "IDENT"),
            Self::IF_BLOCK => ::core::fmt::Formatter::write_str(f, "IF_BLOCK"),
            Self::IF_EXPR => ::core::fmt::Formatter::write_str(f, "IF_EXPR"),
            Self::IF_KW => ::core::fmt::Formatter::write_str(f, "if"),
            Self::IMPL_BLOCK => ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK"),
            Self::IMPL_BLOCK_CONTENTS => {
                ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK_CONTENTS")
            }
            Self::IMPL_KW => ::core::fmt::Formatter::write_str(f, "impl"),
            Self::IN_KW => ::core::fmt::Formatter::write_str(f, "in"),
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LET_KW => ::core::fmt::Formatter::write_str(f, "let"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
            Self::LOOP_EXPR => ::core::fmt::Formatter::write_str(f, "LOOP_EXPR"),
            Self::LOOP_KW => ::core::fmt::Formatter::write_str(f, "loop"),
            Self::L_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '<'),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "<="),
            Self::L_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '['),
            Self::L_CURLY => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '{'),
            Self::L_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '('),
            Self::MATCH_ARM => ::core::fmt::Formatter::write_str(f, "MATCH_ARM"),
            Self::MATCH_EXPR => ::core::fmt::Formatter::write_str(f, "MATCH_EXPR"),
            Self::MATCH_KW => ::core::fmt::Formatter::write_str(f, "match"),
            Self::MINUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '-'),
            Self::MINUS_EQ => ::core::fmt::Formatter::write_str(f, "-="),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::NEQ => ::core::fmt::Formatter::write_str(f, "!="),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::NUMBER_LITERAL => ::core::fmt::Formatter::write_str(f, "number_literal"),
            Self::OR => ::core::fmt::Formatter::write_str(f, "OR"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATH => ::core::fmt::Formatter::write_str(f, "PATH"),
            Self::PATH_SEGMENT => ::core::fmt::Formatter::write_str(f, "PATH_SEGMENT"),
            Self::PATH_TAIL => ::core::fmt::Formatter::write_str(f, "PATH_TAIL"),
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
            Self::PERCENT => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '%'),
            Self::PERCENT_EQ => ::core::fmt::Formatter::write_str(f, "%="),
            Self::PIPE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '|'),
            Self::PIPE_EQ => ::core::fmt::Formatter::write_str(f, "|="),
            Self::PLUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '+'),
            Self::PLUS_EQ => ::core::fmt::Formatter::write_str(f, "+="),
            Self::PUB_KW => ::core::fmt::Formatter::write_str(f, "pub"),
            Self::QUALIFIED_REF => ::core::fmt::Formatter::write_str(f, "QUALIFIED_REF"),
            Self::RETURN_KW => ::core::fmt::Formatter::write_str(f, "return"),
            Self::RET_EXPR => ::core::fmt::Formatter::write_str(f, "RET_EXPR"),
            Self::RIGHT_ARROW => ::core::fmt::Formatter::write_str(f, "->"),
            Self::RIGHT_ROCKET => ::core::fmt::Formatter::write_str(f, "=>"),
            Self::ROOT => ::core::fmt::Formatter::write_str(f, "ROOT"),
            Self::R_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '>'),
            Self::R_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, ">="),
            Self::R_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ']'),
            Self::R_CURLY => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '}'),
            Self::R_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ')'),
            Self::SEMICOLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ';'),
            Self::SHL => ::core::fmt::Formatter::write_str(f, "<<"),
            Self::SHL_EQ => ::core::fmt::Formatter::write_str(f, "<<="),
            Self::SHR => ::core::fmt::Formatter::write_str(f, ">>"),
            Self::SHR_EQ => ::core::fmt::Formatter::write_str(f, ">>="),
            Self::SLASH => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '/'),
            Self::SLASH_EQ => ::core::fmt::Formatter::write_str(f, "/="),
            Self::STAR => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '*'),
            Self::STAR_EQ => ::core::fmt::Formatter::write_str(f, "*="),
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRING_LITERAL => ::core::fmt::Formatter::write_str(f, "string_literal"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_FIELDS => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELDS"),
            Self::STRUCT_FIELD_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELD_NAME"),
            Self::STRUCT_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_EXPR"),
            Self::STRUCT_INIT_FIELD => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_FIELD"),
            Self::STRUCT_KW => ::core::fmt::Formatter::write_str(f, "struct"),
            Self::STRUCT_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_NAME"),
            Self::STRUCT_PATTERN => ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN"),
            Self::STRUCT_PATTERN_FIELD => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD")
            }
            Self::STRUCT_PATTERN_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD_NAME")
            }
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE_KW => ::core::fmt::Formatter::write_str(f, "true"),
            Self::TUPLE_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_EXPR_ELEM"),
            Self::TUPLE_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "TUPLE_INIT_EXPR"),
            Self::TUPLE_PATTERN => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN"),
            Self::TUPLE_PATTERN_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN_ELEM"),
            Self::TUPLE_STRUCT_FIELD => ::core::fmt::Formatter::write_str(f, "TUPLE_STRUCT_FIELD"),
            Self::TUPLE_STRUCT_FIELDS => {
                ::core::fmt::Formatter::write_str(f, "TUPLE_STRUCT_FIELDS")
            }
            Self::TUPLE_TYPE => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE"),
            Self::TUPLE_TYPE_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE_ELEM"),
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::TYPE_ALIAS => ::core::fmt::Formatter::write_str(f, "TYPE_ALIAS"),
            Self::TYPE_KW => ::core::fmt::Formatter::write_str(f, "type"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_ALIAS_NAME => ::core::fmt::Formatter::write_str(f, "USE_ALIAS_NAME"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_BRANCH_OR_ALIAS => {
                ::core::fmt::Formatter::write_str(f, "USE_BRANCH_OR_ALIAS")
            }
            Self::USE_DEF => ::core::fmt::Formatter::write_str(f, "USE_DEF"),
            Self::USE_KW => ::core::fmt::Formatter::write_str(f, "use"),
            Self::USE_TREE => ::core::fmt::Formatter::write_str(f, "USE_TREE"),
            Self::VARIANT_STRUCT => ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT"),
            Self::VARIANT_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD")
            }
            Self::VARIANT_STRUCT_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD_NAME")
            }
            Self::VARIANT_TUPLE => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE"),
            Self::VARIANT_TUPLE_ELEM => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE_ELEM"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::WHILE_EXPR => ::core::fmt::Formatter::write_str(f, "WHILE_EXPR"),
            Self::WHILE_KW => ::core::fmt::Formatter::write_str(f, "while"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
        }
    }
}
impl ::core::convert::From<SyntaxKind> for ::cstree::SyntaxKind {
    #[inline]
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
impl ::core::convert::From<SyntaxKind> for u16 {
    #[inline]
    fn from(kind: SyntaxKind) -> Self {
        kind as u16
    }
}
const _: () = {
    #[cold]
    #[track_caller]
    #[inline(never)]
    fn invalid_syntax_kind(kind: u16) -> ! {
        ::core::panic!(
            "invalid SyntaxKind '{}', must be within the range of 0..=177",
            kind,
        )
    }
    impl ::core::convert::From<u16> for SyntaxKind {
        #[inline]
        #[track_caller]
        fn from(kind: u16) -> Self {
            if kind > 177u16 {
                invalid_syntax_kind(kind)
            } else {
                unsafe { ::core::mem::transmute::<u16, Self>(kind) }
            }
        }
    }
    impl ::core::convert::From<::cstree::SyntaxKind> for SyntaxKind {
        #[inline]
        #[track_caller]
        fn from(::cstree::SyntaxKind(kind): ::cstree::SyntaxKind) -> Self {
            if kind > 177u16 {
                invalid_syntax_kind(kind)
            } else {
                unsafe { ::core::mem::transmute::<u16, Self>(kind) }
            }
        }
    }
};
impl ::core::clone::Clone for SyntaxKind {
    #[inline]
    #[must_use]
    fn clone(&self) -> Self {
        *self
    }
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}
impl ::core::marker::Copy for SyntaxKind {}
impl ::core::hash::Hash for SyntaxKind {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: ::core::hash::Hasher,
    {
        ::core::hash::Hasher::write_u16(state, *self as u16);
    }
}
impl ::core::cmp::PartialEq<SyntaxKind> for SyntaxKind {
    #[inline]
    #[must_use]
    fn eq(&self, other: &SyntaxKind) -> bool {
        *self as u16 == *other as u16
    }
}
impl ::core::cmp::Eq for SyntaxKind {}
impl ::core::cmp::PartialOrd<SyntaxKind> for SyntaxKind {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, other: &SyntaxKind) -> ::core::option::Option<::core::cmp::Ordering> {
        (*self as u16).partial_cmp(&(*other as u16))
    }
}
impl ::core::cmp::Ord for SyntaxKind {
    #[inline]
    #[must_use]
    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
        (*self as u16).cmp(&(*other as u16))
    }
}
/// A convenience macro to create tokens with
#[macro_export]
macro_rules! T {
    (&) => {
        $crate::SyntaxKind::AMPERSAND
    };
    (&=) => {
        $crate::SyntaxKind::AMPERSAND_EQ
    };
    (as) => {
        $crate::SyntaxKind::AS_KW
    };
    (!) => {
        $crate::SyntaxKind::BANG
    };
    (break) => {
        $crate::SyntaxKind::BREAK_KW
    };
    (^) => {
        $crate::SyntaxKind::CARET
    };
    (^=) => {
        $crate::SyntaxKind::CARET_EQ
    };
    (:) => {
        $crate::SyntaxKind::COLON
    };
    (,) => {
        $crate::SyntaxKind::COMMA
    };
    (const) => {
        $crate::SyntaxKind::CONST_KW
    };
    (continue) => {
        $crate::SyntaxKind::CONTINUE_KW
    };
    (.) => {
        $crate::SyntaxKind::DOT
    };
    (::) => {
        $crate::SyntaxKind::DOUBLE_COLON
    };
    (else) => {
        $crate::SyntaxKind::ELSE_KW
    };
    (enum) => {
        $crate::SyntaxKind::ENUM_KW
    };
    (=) => {
        $crate::SyntaxKind::EQ
    };
    (==) => {
        $crate::SyntaxKind::EQEQ
    };
    (false) => {
        $crate::SyntaxKind::FALSE_KW
    };
    (fn) => {
        $crate::SyntaxKind::FN_KW
    };
    (for) => {
        $crate::SyntaxKind::FOR_KW
    };
    ("#[") => {
        $crate::SyntaxKind::HASH_BRACK
    };
    (if) => {
        $crate::SyntaxKind::IF_KW
    };
    (impl) => {
        $crate::SyntaxKind::IMPL_KW
    };
    (in) => {
        $crate::SyntaxKind::IN_KW
    };
    (let) => {
        $crate::SyntaxKind::LET_KW
    };
    (loop) => {
        $crate::SyntaxKind::LOOP_KW
    };
    (<) => {
        $crate::SyntaxKind::L_ANGLE
    };
    (<=) => {
        $crate::SyntaxKind::L_ANGLE_EQ
    };
    ('[') => {
        $crate::SyntaxKind::L_BRACK
    };
    ('{') => {
        $crate::SyntaxKind::L_CURLY
    };
    ('(') => {
        $crate::SyntaxKind::L_PAREN
    };
    (match) => {
        $crate::SyntaxKind::MATCH_KW
    };
    (-) => {
        $crate::SyntaxKind::MINUS
    };
    (-=) => {
        $crate::SyntaxKind::MINUS_EQ
    };
    (!=) => {
        $crate::SyntaxKind::NEQ
    };
    (number_literal) => {
        $crate::SyntaxKind::NUMBER_LITERAL
    };
    (%) => {
        $crate::SyntaxKind::PERCENT
    };
    (%=) => {
        $crate::SyntaxKind::PERCENT_EQ
    };
    (|) => {
        $crate::SyntaxKind::PIPE
    };
    (|=) => {
        $crate::SyntaxKind::PIPE_EQ
    };
    (+) => {
        $crate::SyntaxKind::PLUS
    };
    (+=) => {
        $crate::SyntaxKind::PLUS_EQ
    };
    (pub) => {
        $crate::SyntaxKind::PUB_KW
    };
    (return) => {
        $crate::SyntaxKind::RETURN_KW
    };
    (->) => {
        $crate::SyntaxKind::RIGHT_ARROW
    };
    (=>) => {
        $crate::SyntaxKind::RIGHT_ROCKET
    };
    (>) => {
        $crate::SyntaxKind::R_ANGLE
    };
    (>=) => {
        $crate::SyntaxKind::R_ANGLE_EQ
    };
    (']') => {
        $crate::SyntaxKind::R_BRACK
    };
    ('}') => {
        $crate::SyntaxKind::R_CURLY
    };
    (')') => {
        $crate::SyntaxKind::R_PAREN
    };
    (;) => {
        $crate::SyntaxKind::SEMICOLON
    };
    (<<) => {
        $crate::SyntaxKind::SHL
    };
    (<<=) => {
        $crate::SyntaxKind::SHL_EQ
    };
    (>>) => {
        $crate::SyntaxKind::SHR
    };
    (>>=) => {
        $crate::SyntaxKind::SHR_EQ
    };
    (/) => {
        $crate::SyntaxKind::SLASH
    };
    (/=) => {
        $crate::SyntaxKind::SLASH_EQ
    };
    (*) => {
        $crate::SyntaxKind::STAR
    };
    (*=) => {
        $crate::SyntaxKind::STAR_EQ
    };
    (string_literal) => {
        $crate::SyntaxKind::STRING_LITERAL
    };
    (struct) => {
        $crate::SyntaxKind::STRUCT_KW
    };
    (true) => {
        $crate::SyntaxKind::TRUE_KW
    };
    (type) => {
        $crate::SyntaxKind::TYPE_KW
    };
    (use) => {
        $crate::SyntaxKind::USE_KW
    };
    (while) => {
        $crate::SyntaxKind::WHILE_KW
    };
}
