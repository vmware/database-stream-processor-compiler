#[derive(logos :: Logos)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token("&")]
    AMPERSAND = 0u16,
    #[token("&=")]
    AMPERSAND_EQ = 1u16,
    #[token("and")]
    AND_TOKEN = 2u16,
    ARRAY_ACCESS = 3u16,
    ARRAY_EXPR_ELEM = 4u16,
    ARRAY_INIT_EXPR = 5u16,
    ASSIGN = 6u16,
    ASSIGN_OP = 7u16,
    #[token("as")]
    AS_TOKEN = 8u16,
    ATTRIBUTE = 9u16,
    ATTR_PAIR = 10u16,
    #[token("!")]
    BANG = 11u16,
    BIN_EXPR = 12u16,
    BIN_OP = 13u16,
    BLOCK = 14u16,
    BOOL = 15u16,
    BRACKETED_STRUCT_FIELD = 16u16,
    BRACKETED_STRUCT_FIELDS = 17u16,
    BREAK_EXPR = 18u16,
    #[token("break")]
    BREAK_TOKEN = 19u16,
    #[token("^")]
    CARET = 20u16,
    #[token("^=")]
    CARET_EQ = 21u16,
    CHAR = 22u16,
    #[regex("b?'[^']*'")]
    CHAR_LITERAL = 23u16,
    CLOSURE_ARG = 24u16,
    CLOSURE_EXPR = 25u16,
    #[token(":")]
    COLON = 26u16,
    #[token(",")]
    COMMA = 27u16,
    #[regex("//.*")]
    #[regex("///.*")]
    #[token("/*", lex_block_comment)]
    COMMENT = 28u16,
    CONST_DEF = 29u16,
    #[token("const")]
    CONST_TOKEN = 30u16,
    CONTINUE_EXPR = 31u16,
    #[token("continue")]
    CONTINUE_TOKEN = 32u16,
    #[token(".")]
    DOT = 33u16,
    #[token("::")]
    DOUBLE_COLON = 34u16,
    #[token("..")]
    DOUBLE_DOT = 35u16,
    #[token("..=")]
    DOUBLE_DOT_EQ = 36u16,
    ELSE_BLOCK = 37u16,
    #[token("else")]
    ELSE_TOKEN = 38u16,
    ENUM_DEF = 39u16,
    #[token("enum")]
    ENUM_TOKEN = 40u16,
    ENUM_VARIANT = 41u16,
    ENUM_VARIANTS = 42u16,
    ENUM_VARIANT_BODY = 43u16,
    #[token("eof")]
    EOF = 44u16,
    #[token("=")]
    EQ = 45u16,
    #[token("==")]
    EQEQ = 46u16,
    #[error]
    ERROR = 47u16,
    EXPR = 48u16,
    EXPR_STMT = 49u16,
    #[token("false")]
    FALSE_TOKEN = 50u16,
    FIELD_ACCESS = 51u16,
    FIELD_ACCESSOR = 52u16,
    FIELD_ACCESSOR_NAME = 53u16,
    #[token("fn")]
    FN_TOKEN = 54u16,
    FOR_EXPR = 55u16,
    #[token("for")]
    FOR_TOKEN = 56u16,
    FUNCTION_ARG = 57u16,
    FUNCTION_ARGS = 58u16,
    FUNCTION_CALL = 59u16,
    FUNCTION_CALL_ARG = 60u16,
    FUNCTION_DEF = 61u16,
    FUNCTION_RETURN = 62u16,
    FUNCTION_RETURN_TYPE = 63u16,
    FUNCTION_TYPE = 64u16,
    FUNCTION_TYPE_ARG = 65u16,
    FUNCTION_TYPE_ARGS = 66u16,
    GENERICS = 67u16,
    GENERIC_ARG = 68u16,
    GENERIC_TYPE = 69u16,
    #[token("#[")]
    HASH_BRACK = 70u16,
    #[regex("[A-Za-z_'][A-Za-z0-9_']*")]
    IDENT = 71u16,
    IF_BLOCK = 72u16,
    IF_EXPR = 73u16,
    #[token("if")]
    IF_TOKEN = 74u16,
    IMPL_BLOCK = 75u16,
    IMPL_BLOCK_CONTENTS = 76u16,
    #[token("impl")]
    IMPL_TOKEN = 77u16,
    #[token("in")]
    IN_TOKEN = 78u16,
    ITEM = 79u16,
    #[token("let")]
    LET_TOKEN = 80u16,
    LITERAL = 81u16,
    LOOP_EXPR = 82u16,
    #[token("loop")]
    LOOP_TOKEN = 83u16,
    #[token("<")]
    L_ANGLE = 84u16,
    #[token("<=")]
    L_ANGLE_EQ = 85u16,
    #[token("[")]
    L_BRACK = 86u16,
    #[token("{")]
    L_CURLY = 87u16,
    #[token("(")]
    L_PAREN = 88u16,
    MATCH_ARM = 89u16,
    MATCH_EXPR = 90u16,
    #[token("match")]
    MATCH_TOKEN = 91u16,
    METHOD_CALL = 92u16,
    METHOD_CALL_ARG = 93u16,
    #[token("-")]
    MINUS = 94u16,
    #[token("-=")]
    MINUS_EQ = 95u16,
    MODIFIER = 96u16,
    #[token("!=")]
    NEQ = 97u16,
    NUMBER = 98u16,
    #[regex("[0-9][0-9_]*")]
    #[regex("0b[0-1][0-1_]*")]
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*")]
    NUMBER_LITERAL = 99u16,
    #[token("or")]
    OR_TOKEN = 100u16,
    PAREN_EXPR = 101u16,
    PATH = 102u16,
    PATH_TAIL = 103u16,
    PATTERN = 104u16,
    #[token("%")]
    PERCENT = 105u16,
    #[token("%=")]
    PERCENT_EQ = 106u16,
    #[token("|")]
    PIPE = 107u16,
    #[token("|=")]
    PIPE_EQ = 108u16,
    #[token("+")]
    PLUS = 109u16,
    #[token("+=")]
    PLUS_EQ = 110u16,
    #[token("pub")]
    PUB_TOKEN = 111u16,
    QUALIFIED_REF = 112u16,
    RANGE_EXPR = 113u16,
    RANGE_OP = 114u16,
    #[token("return")]
    RETURN_TOKEN = 115u16,
    RET_EXPR = 116u16,
    #[token("->")]
    RIGHT_ARROW = 117u16,
    #[token("=>")]
    RIGHT_ROCKET = 118u16,
    ROOT = 119u16,
    #[token(">")]
    R_ANGLE = 120u16,
    #[token(">=")]
    R_ANGLE_EQ = 121u16,
    #[token("]")]
    R_BRACK = 122u16,
    #[token("}")]
    R_CURLY = 123u16,
    #[token(")")]
    R_PAREN = 124u16,
    #[token(";")]
    SEMICOLON = 125u16,
    #[token("<<")]
    SHL = 126u16,
    #[token("<<=")]
    SHL_EQ = 127u16,
    #[token(">>")]
    SHR = 128u16,
    #[token(">>=")]
    SHR_EQ = 129u16,
    #[token("/")]
    SLASH = 130u16,
    #[token("/=")]
    SLASH_EQ = 131u16,
    #[token("*")]
    STAR = 132u16,
    #[token("*=")]
    STAR_EQ = 133u16,
    STMT = 134u16,
    STRING = 135u16,
    #[regex(r#"b?"(\\.|[^\\"])*""#)]
    STRING_LITERAL = 136u16,
    STRUCT_DEF = 137u16,
    STRUCT_FIELDS = 138u16,
    STRUCT_INIT_EXPR = 139u16,
    STRUCT_INIT_FIELD = 140u16,
    STRUCT_PATTERN = 141u16,
    STRUCT_PATTERN_FIELD = 142u16,
    #[token("struct")]
    STRUCT_TOKEN = 143u16,
    #[token("tombstone")]
    TOMBSTONE = 144u16,
    #[token("true")]
    TRUE_TOKEN = 145u16,
    TUPLE_EXPR_ELEM = 146u16,
    TUPLE_INIT_EXPR = 147u16,
    TUPLE_PATTERN = 148u16,
    TUPLE_PATTERN_ELEM = 149u16,
    TUPLE_STRUCT_FIELD = 150u16,
    TUPLE_STRUCT_FIELDS = 151u16,
    TUPLE_TYPE = 152u16,
    TUPLE_TYPE_ELEM = 153u16,
    TYPE = 154u16,
    TYPE_ALIAS = 155u16,
    #[token("type")]
    TYPE_TOKEN = 156u16,
    UNARY_EXPR = 157u16,
    UNARY_OP = 158u16,
    USE_ALIAS = 159u16,
    USE_BRANCH = 160u16,
    USE_BRANCH_OR_ALIAS = 161u16,
    USE_DEF = 162u16,
    #[token("use")]
    USE_TOKEN = 163u16,
    VARIANT_STRUCT = 164u16,
    VARIANT_STRUCT_FIELD = 165u16,
    VARIANT_TUPLE = 166u16,
    VARIANT_TUPLE_ELEM = 167u16,
    VAR_DECL = 168u16,
    VAR_REF = 169u16,
    WHILE_EXPR = 170u16,
    #[token("while")]
    WHILE_TOKEN = 171u16,
    #[regex("[\n\t\r ]+")]
    WHITESPACE = 172u16,
}
impl SyntaxKind {
    /// The maximum discriminant of the [`SyntaxKind`] enum
    pub const MAXIMUM_DISCRIMINANT: u16 = 172u16;
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
            Self::CHAR => ::core::fmt::Formatter::write_str(f, "CHAR"),
            Self::CHAR_LITERAL => ::core::fmt::Formatter::write_str(f, "CHAR_LITERAL"),
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
            Self::ENUM_VARIANT_BODY => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_BODY"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EQ => ::core::fmt::Formatter::write_str(f, "EQ"),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "EQEQ"),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "ERROR"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FALSE_TOKEN => ::core::fmt::Formatter::write_str(f, "FALSE_TOKEN"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR"),
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
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LET_TOKEN => ::core::fmt::Formatter::write_str(f, "LET_TOKEN"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
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
            Self::METHOD_CALL => ::core::fmt::Formatter::write_str(f, "METHOD_CALL"),
            Self::METHOD_CALL_ARG => ::core::fmt::Formatter::write_str(f, "METHOD_CALL_ARG"),
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
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
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
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRING_LITERAL => ::core::fmt::Formatter::write_str(f, "STRING_LITERAL"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_FIELDS => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELDS"),
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
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::TYPE_ALIAS => ::core::fmt::Formatter::write_str(f, "TYPE_ALIAS"),
            Self::TYPE_TOKEN => ::core::fmt::Formatter::write_str(f, "TYPE_TOKEN"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_BRANCH_OR_ALIAS => {
                ::core::fmt::Formatter::write_str(f, "USE_BRANCH_OR_ALIAS")
            }
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
            Self::CHAR => ::core::fmt::Formatter::write_str(f, "CHAR"),
            Self::CHAR_LITERAL => ::core::fmt::Formatter::write_str(f, "CHAR_LITERAL"),
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
            Self::ENUM_VARIANT_BODY => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_BODY"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EQ => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '='),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "=="),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "???"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FALSE_TOKEN => ::core::fmt::Formatter::write_str(f, "false"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR"),
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
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LET_TOKEN => ::core::fmt::Formatter::write_str(f, "let"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
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
            Self::METHOD_CALL => ::core::fmt::Formatter::write_str(f, "METHOD_CALL"),
            Self::METHOD_CALL_ARG => ::core::fmt::Formatter::write_str(f, "METHOD_CALL_ARG"),
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
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
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
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRING_LITERAL => ::core::fmt::Formatter::write_str(f, "STRING_LITERAL"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_FIELDS => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELDS"),
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
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::TYPE_ALIAS => ::core::fmt::Formatter::write_str(f, "TYPE_ALIAS"),
            Self::TYPE_TOKEN => ::core::fmt::Formatter::write_str(f, "type"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_BRANCH_OR_ALIAS => {
                ::core::fmt::Formatter::write_str(f, "USE_BRANCH_OR_ALIAS")
            }
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
            "invalid SyntaxKind '{}', must be within the range of 0..=172",
            kind,
        )
    }
    impl ::core::convert::From<u16> for SyntaxKind {
        #[inline]
        #[track_caller]
        fn from(kind: u16) -> Self {
            if kind > 172u16 {
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
            if kind > 172u16 {
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
