#[derive(logos :: Logos)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    AMPERSAND = 0u16,
    AMPERSAND_EQ = 1u16,
    AND_TOKEN = 2u16,
    ARRAY_ACCESS = 3u16,
    ARRAY_EXPR_ELEM = 4u16,
    ARRAY_INIT_EXPR = 5u16,
    ASSIGN = 6u16,
    ASSIGN_OP = 7u16,
    AS_TOKEN = 8u16,
    ATTRIBUTE = 9u16,
    ATTR_PAIR = 10u16,
    BANG = 11u16,
    BIN_EXPR = 12u16,
    BIN_OP = 13u16,
    BLOCK = 14u16,
    BOOL = 15u16,
    BRACKETED_STRUCT_FIELD = 16u16,
    BRACKETED_STRUCT_FIELDS = 17u16,
    BREAK_EXPR = 18u16,
    BREAK_TOKEN = 19u16,
    CARET = 20u16,
    CARET_EQ = 21u16,
    CLOSURE_ARG = 22u16,
    CLOSURE_EXPR = 23u16,
    COLON = 24u16,
    COMMA = 25u16,
    #[regex("//.*")]
    #[regex("///.*")]
    #[token("/*", lex_block_comment)]
    COMMENT = 26u16,
    CONST_DEF = 27u16,
    CONST_TOKEN = 28u16,
    CONTINUE_EXPR = 29u16,
    CONTINUE_TOKEN = 30u16,
    DOT = 31u16,
    DOUBLE_COLON = 32u16,
    DOUBLE_DOT = 33u16,
    DOUBLE_DOT_EQ = 34u16,
    ELSE_BLOCK = 35u16,
    ELSE_TOKEN = 36u16,
    ENUM_DEF = 37u16,
    ENUM_TOKEN = 38u16,
    ENUM_VARIANT = 39u16,
    ENUM_VARIANTS = 40u16,
    EOF = 41u16,
    EQ = 42u16,
    EQEQ = 43u16,
    #[error]
    ERROR = 44u16,
    EXPR_STMT = 45u16,
    FALSE_TOKEN = 46u16,
    FIELD_ACCESS = 47u16,
    FIELD_ACCESSOR_NAME = 48u16,
    FN_TOKEN = 49u16,
    FOR_EXPR = 50u16,
    FOR_TOKEN = 51u16,
    FUNCTION_ARG = 52u16,
    FUNCTION_ARGS = 53u16,
    FUNCTION_CALL = 54u16,
    FUNCTION_CALL_ARG = 55u16,
    FUNCTION_DEF = 56u16,
    FUNCTION_RETURN = 57u16,
    FUNCTION_RETURN_TYPE = 58u16,
    FUNCTION_TYPE = 59u16,
    FUNCTION_TYPE_ARG = 60u16,
    FUNCTION_TYPE_ARGS = 61u16,
    GENERICS = 62u16,
    GENERIC_ARG = 63u16,
    GENERIC_TYPE = 64u16,
    HASH_BRACK = 65u16,
    #[regex("[A-Za-z_'][A-Za-z0-9_']*")]
    IDENT = 66u16,
    IF_BLOCK = 67u16,
    IF_EXPR = 68u16,
    IF_TOKEN = 69u16,
    IMPL_BLOCK = 70u16,
    IMPL_BLOCK_CONTENTS = 71u16,
    IMPL_TOKEN = 72u16,
    IN_TOKEN = 73u16,
    LET_TOKEN = 74u16,
    LOOP_EXPR = 75u16,
    LOOP_TOKEN = 76u16,
    L_ANGLE = 77u16,
    L_ANGLE_EQ = 78u16,
    L_BRACK = 79u16,
    L_CURLY = 80u16,
    L_PAREN = 81u16,
    MATCH_ARM = 82u16,
    MATCH_EXPR = 83u16,
    MATCH_TOKEN = 84u16,
    MINUS = 85u16,
    MINUS_EQ = 86u16,
    MODIFIER = 87u16,
    NEQ = 88u16,
    NUMBER = 89u16,
    #[regex("[0-9][0-9_]*")]
    #[regex("0b[0-1][0-1_]*")]
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*")]
    NUMBER_LITERAL = 90u16,
    OR_TOKEN = 91u16,
    PAREN_EXPR = 92u16,
    PATH = 93u16,
    PATH_TAIL = 94u16,
    PERCENT = 95u16,
    PERCENT_EQ = 96u16,
    PIPE = 97u16,
    PIPE_EQ = 98u16,
    PLUS = 99u16,
    PLUS_EQ = 100u16,
    PUB_TOKEN = 101u16,
    QUALIFIED_REF = 102u16,
    RANGE_EXPR = 103u16,
    RANGE_OP = 104u16,
    RETURN_TOKEN = 105u16,
    RET_EXPR = 106u16,
    RIGHT_ARROW = 107u16,
    RIGHT_ROCKET = 108u16,
    ROOT = 109u16,
    R_ANGLE = 110u16,
    R_ANGLE_EQ = 111u16,
    R_BRACK = 112u16,
    R_CURLY = 113u16,
    R_PAREN = 114u16,
    SEMICOLON = 115u16,
    SHL = 116u16,
    SHL_EQ = 117u16,
    SHR = 118u16,
    SHR_EQ = 119u16,
    SLASH = 120u16,
    SLASH_EQ = 121u16,
    STAR = 122u16,
    STAR_EQ = 123u16,
    STRING = 124u16,
    STRING_LITERAL = 125u16,
    STRUCT_DEF = 126u16,
    STRUCT_INIT_EXPR = 127u16,
    STRUCT_INIT_FIELD = 128u16,
    STRUCT_PATTERN = 129u16,
    STRUCT_PATTERN_FIELD = 130u16,
    STRUCT_TOKEN = 131u16,
    TOMBSTONE = 132u16,
    TRUE_TOKEN = 133u16,
    TUPLE_EXPR_ELEM = 134u16,
    TUPLE_INIT_EXPR = 135u16,
    TUPLE_PATTERN = 136u16,
    TUPLE_PATTERN_ELEM = 137u16,
    TUPLE_STRUCT_FIELD = 138u16,
    TUPLE_STRUCT_FIELDS = 139u16,
    TUPLE_TYPE = 140u16,
    TUPLE_TYPE_ELEM = 141u16,
    TYPE_ALIAS = 142u16,
    TYPE_TOKEN = 143u16,
    UNARY_EXPR = 144u16,
    UNARY_OP = 145u16,
    USE_ALIAS = 146u16,
    USE_BRANCH = 147u16,
    USE_DEF = 148u16,
    USE_TOKEN = 149u16,
    VARIANT_STRUCT = 150u16,
    VARIANT_STRUCT_FIELD = 151u16,
    VARIANT_TUPLE = 152u16,
    VARIANT_TUPLE_ELEM = 153u16,
    VAR_DECL = 154u16,
    VAR_REF = 155u16,
    WHILE_EXPR = 156u16,
    WHILE_TOKEN = 157u16,
    #[regex("[\n\t\r ]+")]
    WHITESPACE = 158u16,
}
impl SyntaxKind {
    /// The maximum discriminant of the [`SyntaxKind`] enum
    pub const MAXIMUM_DISCRIMINANT: u16 = 158u16;
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
            Self::AND_TOKEN => ::core::fmt::Formatter::write_str(f, "AND_TOKEN"),
            Self::ARRAY_ACCESS => ::core::fmt::Formatter::write_str(f, "ARRAY_ACCESS"),
            Self::ARRAY_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "ARRAY_EXPR_ELEM"),
            Self::ARRAY_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "ARRAY_INIT_EXPR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ASSIGN_OP => ::core::fmt::Formatter::write_str(f, "ASSIGN_OP"),
            Self::AS_TOKEN => ::core::fmt::Formatter::write_str(f, "AS_TOKEN"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
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
            Self::BREAK_TOKEN => ::core::fmt::Formatter::write_str(f, "BREAK_TOKEN"),
            Self::CARET => ::core::fmt::Formatter::write_str(f, "CARET"),
            Self::CARET_EQ => ::core::fmt::Formatter::write_str(f, "CARET_EQ"),
            Self::CLOSURE_ARG => ::core::fmt::Formatter::write_str(f, "CLOSURE_ARG"),
            Self::CLOSURE_EXPR => ::core::fmt::Formatter::write_str(f, "CLOSURE_EXPR"),
            Self::COLON => ::core::fmt::Formatter::write_str(f, "COLON"),
            Self::COMMA => ::core::fmt::Formatter::write_str(f, "COMMA"),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::CONST_DEF => ::core::fmt::Formatter::write_str(f, "CONST_DEF"),
            Self::CONST_TOKEN => ::core::fmt::Formatter::write_str(f, "CONST_TOKEN"),
            Self::CONTINUE_EXPR => ::core::fmt::Formatter::write_str(f, "CONTINUE_EXPR"),
            Self::CONTINUE_TOKEN => ::core::fmt::Formatter::write_str(f, "CONTINUE_TOKEN"),
            Self::DOT => ::core::fmt::Formatter::write_str(f, "DOT"),
            Self::DOUBLE_COLON => ::core::fmt::Formatter::write_str(f, "DOUBLE_COLON"),
            Self::DOUBLE_DOT => ::core::fmt::Formatter::write_str(f, "DOUBLE_DOT"),
            Self::DOUBLE_DOT_EQ => ::core::fmt::Formatter::write_str(f, "DOUBLE_DOT_EQ"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::ELSE_TOKEN => ::core::fmt::Formatter::write_str(f, "ELSE_TOKEN"),
            Self::ENUM_DEF => ::core::fmt::Formatter::write_str(f, "ENUM_DEF"),
            Self::ENUM_TOKEN => ::core::fmt::Formatter::write_str(f, "ENUM_TOKEN"),
            Self::ENUM_VARIANT => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT"),
            Self::ENUM_VARIANTS => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANTS"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EQ => ::core::fmt::Formatter::write_str(f, "EQ"),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "EQEQ"),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "ERROR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FALSE_TOKEN => ::core::fmt::Formatter::write_str(f, "FALSE_TOKEN"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR_NAME => {
                ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR_NAME")
            }
            Self::FN_TOKEN => ::core::fmt::Formatter::write_str(f, "FN_TOKEN"),
            Self::FOR_EXPR => ::core::fmt::Formatter::write_str(f, "FOR_EXPR"),
            Self::FOR_TOKEN => ::core::fmt::Formatter::write_str(f, "FOR_TOKEN"),
            Self::FUNCTION_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARG"),
            Self::FUNCTION_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARGS"),
            Self::FUNCTION_CALL => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL"),
            Self::FUNCTION_CALL_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL_ARG"),
            Self::FUNCTION_DEF => ::core::fmt::Formatter::write_str(f, "FUNCTION_DEF"),
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
            Self::IF_TOKEN => ::core::fmt::Formatter::write_str(f, "IF_TOKEN"),
            Self::IMPL_BLOCK => ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK"),
            Self::IMPL_BLOCK_CONTENTS => {
                ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK_CONTENTS")
            }
            Self::IMPL_TOKEN => ::core::fmt::Formatter::write_str(f, "IMPL_TOKEN"),
            Self::IN_TOKEN => ::core::fmt::Formatter::write_str(f, "IN_TOKEN"),
            Self::LET_TOKEN => ::core::fmt::Formatter::write_str(f, "LET_TOKEN"),
            Self::LOOP_EXPR => ::core::fmt::Formatter::write_str(f, "LOOP_EXPR"),
            Self::LOOP_TOKEN => ::core::fmt::Formatter::write_str(f, "LOOP_TOKEN"),
            Self::L_ANGLE => ::core::fmt::Formatter::write_str(f, "L_ANGLE"),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "L_ANGLE_EQ"),
            Self::L_BRACK => ::core::fmt::Formatter::write_str(f, "L_BRACK"),
            Self::L_CURLY => ::core::fmt::Formatter::write_str(f, "L_CURLY"),
            Self::L_PAREN => ::core::fmt::Formatter::write_str(f, "L_PAREN"),
            Self::MATCH_ARM => ::core::fmt::Formatter::write_str(f, "MATCH_ARM"),
            Self::MATCH_EXPR => ::core::fmt::Formatter::write_str(f, "MATCH_EXPR"),
            Self::MATCH_TOKEN => ::core::fmt::Formatter::write_str(f, "MATCH_TOKEN"),
            Self::MINUS => ::core::fmt::Formatter::write_str(f, "MINUS"),
            Self::MINUS_EQ => ::core::fmt::Formatter::write_str(f, "MINUS_EQ"),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::NEQ => ::core::fmt::Formatter::write_str(f, "NEQ"),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::NUMBER_LITERAL => ::core::fmt::Formatter::write_str(f, "NUMBER_LITERAL"),
            Self::OR_TOKEN => ::core::fmt::Formatter::write_str(f, "OR_TOKEN"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATH => ::core::fmt::Formatter::write_str(f, "PATH"),
            Self::PATH_TAIL => ::core::fmt::Formatter::write_str(f, "PATH_TAIL"),
            Self::PERCENT => ::core::fmt::Formatter::write_str(f, "PERCENT"),
            Self::PERCENT_EQ => ::core::fmt::Formatter::write_str(f, "PERCENT_EQ"),
            Self::PIPE => ::core::fmt::Formatter::write_str(f, "PIPE"),
            Self::PIPE_EQ => ::core::fmt::Formatter::write_str(f, "PIPE_EQ"),
            Self::PLUS => ::core::fmt::Formatter::write_str(f, "PLUS"),
            Self::PLUS_EQ => ::core::fmt::Formatter::write_str(f, "PLUS_EQ"),
            Self::PUB_TOKEN => ::core::fmt::Formatter::write_str(f, "PUB_TOKEN"),
            Self::QUALIFIED_REF => ::core::fmt::Formatter::write_str(f, "QUALIFIED_REF"),
            Self::RANGE_EXPR => ::core::fmt::Formatter::write_str(f, "RANGE_EXPR"),
            Self::RANGE_OP => ::core::fmt::Formatter::write_str(f, "RANGE_OP"),
            Self::RETURN_TOKEN => ::core::fmt::Formatter::write_str(f, "RETURN_TOKEN"),
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
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRING_LITERAL => ::core::fmt::Formatter::write_str(f, "STRING_LITERAL"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_EXPR"),
            Self::STRUCT_INIT_FIELD => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_FIELD"),
            Self::STRUCT_PATTERN => ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN"),
            Self::STRUCT_PATTERN_FIELD => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD")
            }
            Self::STRUCT_TOKEN => ::core::fmt::Formatter::write_str(f, "STRUCT_TOKEN"),
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE_TOKEN => ::core::fmt::Formatter::write_str(f, "TRUE_TOKEN"),
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
            Self::TYPE_ALIAS => ::core::fmt::Formatter::write_str(f, "TYPE_ALIAS"),
            Self::TYPE_TOKEN => ::core::fmt::Formatter::write_str(f, "TYPE_TOKEN"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_DEF => ::core::fmt::Formatter::write_str(f, "USE_DEF"),
            Self::USE_TOKEN => ::core::fmt::Formatter::write_str(f, "USE_TOKEN"),
            Self::VARIANT_STRUCT => ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT"),
            Self::VARIANT_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD")
            }
            Self::VARIANT_TUPLE => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE"),
            Self::VARIANT_TUPLE_ELEM => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE_ELEM"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::WHILE_EXPR => ::core::fmt::Formatter::write_str(f, "WHILE_EXPR"),
            Self::WHILE_TOKEN => ::core::fmt::Formatter::write_str(f, "WHILE_TOKEN"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
        }
    }
}
impl ::core::fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::AMPERSAND => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '&'),
            Self::AMPERSAND_EQ => ::core::fmt::Formatter::write_str(f, "&="),
            Self::AND_TOKEN => ::core::fmt::Formatter::write_str(f, "and"),
            Self::ARRAY_ACCESS => ::core::fmt::Formatter::write_str(f, "ARRAY_ACCESS"),
            Self::ARRAY_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "ARRAY_EXPR_ELEM"),
            Self::ARRAY_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "ARRAY_INIT_EXPR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ASSIGN_OP => ::core::fmt::Formatter::write_str(f, "ASSIGN_OP"),
            Self::AS_TOKEN => ::core::fmt::Formatter::write_str(f, "as"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
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
            Self::BREAK_TOKEN => ::core::fmt::Formatter::write_str(f, "break"),
            Self::CARET => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '^'),
            Self::CARET_EQ => ::core::fmt::Formatter::write_str(f, "^="),
            Self::CLOSURE_ARG => ::core::fmt::Formatter::write_str(f, "CLOSURE_ARG"),
            Self::CLOSURE_EXPR => ::core::fmt::Formatter::write_str(f, "CLOSURE_EXPR"),
            Self::COLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ':'),
            Self::COMMA => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ','),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::CONST_DEF => ::core::fmt::Formatter::write_str(f, "CONST_DEF"),
            Self::CONST_TOKEN => ::core::fmt::Formatter::write_str(f, "const"),
            Self::CONTINUE_EXPR => ::core::fmt::Formatter::write_str(f, "CONTINUE_EXPR"),
            Self::CONTINUE_TOKEN => ::core::fmt::Formatter::write_str(f, "continue"),
            Self::DOT => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '.'),
            Self::DOUBLE_COLON => ::core::fmt::Formatter::write_str(f, "::"),
            Self::DOUBLE_DOT => ::core::fmt::Formatter::write_str(f, ".."),
            Self::DOUBLE_DOT_EQ => ::core::fmt::Formatter::write_str(f, "..="),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::ELSE_TOKEN => ::core::fmt::Formatter::write_str(f, "else"),
            Self::ENUM_DEF => ::core::fmt::Formatter::write_str(f, "ENUM_DEF"),
            Self::ENUM_TOKEN => ::core::fmt::Formatter::write_str(f, "enum"),
            Self::ENUM_VARIANT => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT"),
            Self::ENUM_VARIANTS => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANTS"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EQ => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '='),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "=="),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "???"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FALSE_TOKEN => ::core::fmt::Formatter::write_str(f, "false"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR_NAME => {
                ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR_NAME")
            }
            Self::FN_TOKEN => ::core::fmt::Formatter::write_str(f, "fn"),
            Self::FOR_EXPR => ::core::fmt::Formatter::write_str(f, "FOR_EXPR"),
            Self::FOR_TOKEN => ::core::fmt::Formatter::write_str(f, "for"),
            Self::FUNCTION_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARG"),
            Self::FUNCTION_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARGS"),
            Self::FUNCTION_CALL => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL"),
            Self::FUNCTION_CALL_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_CALL_ARG"),
            Self::FUNCTION_DEF => ::core::fmt::Formatter::write_str(f, "FUNCTION_DEF"),
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
            Self::IF_TOKEN => ::core::fmt::Formatter::write_str(f, "if"),
            Self::IMPL_BLOCK => ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK"),
            Self::IMPL_BLOCK_CONTENTS => {
                ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK_CONTENTS")
            }
            Self::IMPL_TOKEN => ::core::fmt::Formatter::write_str(f, "impl"),
            Self::IN_TOKEN => ::core::fmt::Formatter::write_str(f, "in"),
            Self::LET_TOKEN => ::core::fmt::Formatter::write_str(f, "let"),
            Self::LOOP_EXPR => ::core::fmt::Formatter::write_str(f, "LOOP_EXPR"),
            Self::LOOP_TOKEN => ::core::fmt::Formatter::write_str(f, "loop"),
            Self::L_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '<'),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "<="),
            Self::L_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '['),
            Self::L_CURLY => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '{'),
            Self::L_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '('),
            Self::MATCH_ARM => ::core::fmt::Formatter::write_str(f, "MATCH_ARM"),
            Self::MATCH_EXPR => ::core::fmt::Formatter::write_str(f, "MATCH_EXPR"),
            Self::MATCH_TOKEN => ::core::fmt::Formatter::write_str(f, "match"),
            Self::MINUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '-'),
            Self::MINUS_EQ => ::core::fmt::Formatter::write_str(f, "-="),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::NEQ => ::core::fmt::Formatter::write_str(f, "!="),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::NUMBER_LITERAL => ::core::fmt::Formatter::write_str(f, "NUMBER_LITERAL"),
            Self::OR_TOKEN => ::core::fmt::Formatter::write_str(f, "or"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATH => ::core::fmt::Formatter::write_str(f, "PATH"),
            Self::PATH_TAIL => ::core::fmt::Formatter::write_str(f, "PATH_TAIL"),
            Self::PERCENT => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '%'),
            Self::PERCENT_EQ => ::core::fmt::Formatter::write_str(f, "%="),
            Self::PIPE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '|'),
            Self::PIPE_EQ => ::core::fmt::Formatter::write_str(f, "|="),
            Self::PLUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '+'),
            Self::PLUS_EQ => ::core::fmt::Formatter::write_str(f, "+="),
            Self::PUB_TOKEN => ::core::fmt::Formatter::write_str(f, "pub"),
            Self::QUALIFIED_REF => ::core::fmt::Formatter::write_str(f, "QUALIFIED_REF"),
            Self::RANGE_EXPR => ::core::fmt::Formatter::write_str(f, "RANGE_EXPR"),
            Self::RANGE_OP => ::core::fmt::Formatter::write_str(f, "RANGE_OP"),
            Self::RETURN_TOKEN => ::core::fmt::Formatter::write_str(f, "return"),
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
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRING_LITERAL => ::core::fmt::Formatter::write_str(f, "STRING_LITERAL"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_EXPR"),
            Self::STRUCT_INIT_FIELD => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_FIELD"),
            Self::STRUCT_PATTERN => ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN"),
            Self::STRUCT_PATTERN_FIELD => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD")
            }
            Self::STRUCT_TOKEN => ::core::fmt::Formatter::write_str(f, "struct"),
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE_TOKEN => ::core::fmt::Formatter::write_str(f, "true"),
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
            Self::TYPE_ALIAS => ::core::fmt::Formatter::write_str(f, "TYPE_ALIAS"),
            Self::TYPE_TOKEN => ::core::fmt::Formatter::write_str(f, "type"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_DEF => ::core::fmt::Formatter::write_str(f, "USE_DEF"),
            Self::USE_TOKEN => ::core::fmt::Formatter::write_str(f, "use"),
            Self::VARIANT_STRUCT => ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT"),
            Self::VARIANT_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD")
            }
            Self::VARIANT_TUPLE => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE"),
            Self::VARIANT_TUPLE_ELEM => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE_ELEM"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::WHILE_EXPR => ::core::fmt::Formatter::write_str(f, "WHILE_EXPR"),
            Self::WHILE_TOKEN => ::core::fmt::Formatter::write_str(f, "while"),
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
            "invalid SyntaxKind '{}', must be within the range of 0..=158",
            kind,
        )
    }
    impl ::core::convert::From<u16> for SyntaxKind {
        #[inline]
        #[track_caller]
        fn from(kind: u16) -> Self {
            if kind > 158u16 {
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
            if kind > 158u16 {
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
    (and) => {
        $crate::SyntaxKind::AND_TOKEN
    };
    (as) => {
        $crate::SyntaxKind::AS_TOKEN
    };
    (!) => {
        $crate::SyntaxKind::BANG
    };
    (break) => {
        $crate::SyntaxKind::BREAK_TOKEN
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
        $crate::SyntaxKind::CONST_TOKEN
    };
    (continue) => {
        $crate::SyntaxKind::CONTINUE_TOKEN
    };
    (.) => {
        $crate::SyntaxKind::DOT
    };
    (::) => {
        $crate::SyntaxKind::DOUBLE_COLON
    };
    (..) => {
        $crate::SyntaxKind::DOUBLE_DOT
    };
    (..=) => {
        $crate::SyntaxKind::DOUBLE_DOT_EQ
    };
    (else) => {
        $crate::SyntaxKind::ELSE_TOKEN
    };
    (enum) => {
        $crate::SyntaxKind::ENUM_TOKEN
    };
    (=) => {
        $crate::SyntaxKind::EQ
    };
    (==) => {
        $crate::SyntaxKind::EQEQ
    };
    (false) => {
        $crate::SyntaxKind::FALSE_TOKEN
    };
    (fn) => {
        $crate::SyntaxKind::FN_TOKEN
    };
    (for) => {
        $crate::SyntaxKind::FOR_TOKEN
    };
    ("#[") => {
        $crate::SyntaxKind::HASH_BRACK
    };
    (if) => {
        $crate::SyntaxKind::IF_TOKEN
    };
    (impl) => {
        $crate::SyntaxKind::IMPL_TOKEN
    };
    (in) => {
        $crate::SyntaxKind::IN_TOKEN
    };
    (let) => {
        $crate::SyntaxKind::LET_TOKEN
    };
    (loop) => {
        $crate::SyntaxKind::LOOP_TOKEN
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
        $crate::SyntaxKind::MATCH_TOKEN
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
    (or) => {
        $crate::SyntaxKind::OR_TOKEN
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
        $crate::SyntaxKind::PUB_TOKEN
    };
    (return) => {
        $crate::SyntaxKind::RETURN_TOKEN
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
    (struct) => {
        $crate::SyntaxKind::STRUCT_TOKEN
    };
    (true) => {
        $crate::SyntaxKind::TRUE_TOKEN
    };
    (type) => {
        $crate::SyntaxKind::TYPE_TOKEN
    };
    (use) => {
        $crate::SyntaxKind::USE_TOKEN
    };
    (while) => {
        $crate::SyntaxKind::WHILE_TOKEN
    };
}
