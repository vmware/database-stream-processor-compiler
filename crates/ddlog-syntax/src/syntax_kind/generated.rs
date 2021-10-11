#[derive(logos :: Logos)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token("!")]
    BANG = 0u16,
    #[token("!=")]
    NEQ = 1u16,
    #[token("#[")]
    HASH_BRACK = 2u16,
    #[token("%")]
    PERCENT = 3u16,
    #[token("%=")]
    PERCENT_EQ = 4u16,
    #[token("&")]
    AMPERSAND = 5u16,
    #[token("&=")]
    AMPERSAND_EQ = 6u16,
    #[token("(")]
    L_PAREN = 7u16,
    #[token(")")]
    R_PAREN = 8u16,
    #[token("*")]
    STAR = 9u16,
    #[token("*=")]
    STAR_EQ = 10u16,
    #[token("+")]
    PLUS = 11u16,
    #[token("+=")]
    PLUS_EQ = 12u16,
    #[token(",")]
    COMMA = 13u16,
    #[token("-")]
    MINUS = 14u16,
    #[token("-=")]
    MINUS_EQ = 15u16,
    #[token("->")]
    RIGHT_ARROW = 16u16,
    #[token(".")]
    DOT = 17u16,
    #[token("/")]
    SLASH = 18u16,
    #[token("/=")]
    SLASH_EQ = 19u16,
    #[token(":")]
    COLON = 20u16,
    #[token("::")]
    DOUBLE_COLON = 21u16,
    #[token(";")]
    SEMICOLON = 22u16,
    #[token("<")]
    L_ANGLE = 23u16,
    #[token("<<")]
    SHL = 24u16,
    #[token("<<=")]
    SHL_EQ = 25u16,
    #[token("<=")]
    L_ANGLE_EQ = 26u16,
    #[token("=")]
    EQ = 27u16,
    #[token("==")]
    EQEQ = 28u16,
    #[token("=>")]
    RIGHT_ROCKET = 29u16,
    #[token(">")]
    R_ANGLE = 30u16,
    #[token(">=")]
    R_ANGLE_EQ = 31u16,
    #[token(">>")]
    SHR = 32u16,
    #[token(">>=")]
    SHR_EQ = 33u16,
    ARRAY_ACCESS = 34u16,
    ARRAY_EXPR_ELEM = 35u16,
    ARRAY_INIT_EXPR = 36u16,
    ASSIGN = 37u16,
    ASSIGN_OP = 38u16,
    ATTR_NAME = 39u16,
    ATTR_PAIR = 40u16,
    ATTRIBUTE = 41u16,
    BIN_EXPR = 42u16,
    BIN_OP = 43u16,
    BLOCK = 44u16,
    BOOL = 45u16,
    BRACKED_STRUCT_FIELD = 46u16,
    BRACKED_STRUCT_FIELDS = 47u16,
    BREAK_EXPR = 48u16,
    CLOSURE_ARG = 49u16,
    CLOSURE_EXPR = 50u16,
    CONST_DEF = 51u16,
    CONST_NAME = 52u16,
    CONST_VALUE = 53u16,
    CONTINUE_EXPR = 54u16,
    ELSE_BLOCK = 55u16,
    ENUM_DEF = 56u16,
    ENUM_NAME = 57u16,
    ENUM_VARIANT = 58u16,
    ENUM_VARIANT_BODY = 59u16,
    ENUM_VARIANT_NAME = 60u16,
    ENUM_VARIANTS = 61u16,
    EXPR = 62u16,
    EXPR_STMT = 63u16,
    FIELD_ACCESS = 64u16,
    FIELD_ACCESSOR = 65u16,
    FIELD_ACCESSOR_NAME = 66u16,
    FOR_EXPR = 67u16,
    FUNCTION_ARG = 68u16,
    FUNCTION_ARGS = 69u16,
    FUNCTION_CALL = 70u16,
    FUNCTION_CALL_ARG = 71u16,
    FUNCTION_DEF = 72u16,
    FUNCTION_NAME = 73u16,
    FUNCTION_RETURN = 74u16,
    FUNCTION_RETURN_TYPE = 75u16,
    FUNCTION_TYPE = 76u16,
    FUNCTION_TYPE_ARG = 77u16,
    FUNCTION_TYPE_ARGS = 78u16,
    GENERIC_ARG = 79u16,
    GENERIC_TYPE = 80u16,
    GENERICS = 81u16,
    IF_BLOCK = 82u16,
    IF_EXPR = 83u16,
    IMPL_BLOCK = 84u16,
    IMPL_BLOCK_CONTENTS = 85u16,
    ITEM = 86u16,
    LITERAL = 87u16,
    LOOP_EXPR = 88u16,
    MATCH_ARM = 89u16,
    MATCH_EXPR = 90u16,
    MODIFIER = 91u16,
    PAREN_EXPR = 92u16,
    PATH = 93u16,
    PATH_SEGMENT = 94u16,
    PATH_TAIL = 95u16,
    PATTERN = 96u16,
    QUALIFIED_REF = 97u16,
    RET_EXPR = 98u16,
    ROOT = 99u16,
    STMT = 100u16,
    STRUCT_DEF = 101u16,
    STRUCT_FIELD_NAME = 102u16,
    STRUCT_FIELDS = 103u16,
    STRUCT_INIT_EXPR = 104u16,
    STRUCT_INIT_FIELD = 105u16,
    STRUCT_NAME = 106u16,
    STRUCT_PATTERN = 107u16,
    STRUCT_PATTERN_FIELD = 108u16,
    STRUCT_PATTERN_FIELD_NAME = 109u16,
    TUPLE_EXPR_ELEM = 110u16,
    TUPLE_INIT_EXPR = 111u16,
    TUPLE_PATTERN = 112u16,
    TUPLE_PATTERN_ELEM = 113u16,
    TUPLE_STRUCT_FIELD = 114u16,
    TUPLE_STRUCT_FIELDS = 115u16,
    TUPLE_TYPE = 116u16,
    TUPLE_TYPE_ELEM = 117u16,
    TYPE = 118u16,
    TYPE_ALIAS = 119u16,
    UNARY_EXPR = 120u16,
    UNARY_OP = 121u16,
    USE_ALIAS = 122u16,
    USE_ALIAS_NAME = 123u16,
    USE_BRANCH = 124u16,
    USE_BRANCH_OR_ALIAS = 125u16,
    USE_DEF = 126u16,
    USE_TREE = 127u16,
    VAR_DECL = 128u16,
    VAR_REF = 129u16,
    VARIANT_STRUCT = 130u16,
    VARIANT_STRUCT_FIELD = 131u16,
    VARIANT_STRUCT_FIELD_NAME = 132u16,
    VARIANT_TUPLE = 133u16,
    VARIANT_TUPLE_ELEM = 134u16,
    WHILE_EXPR = 135u16,
    #[token("[")]
    L_BRACK = 136u16,
    #[token("]")]
    R_BRACK = 137u16,
    #[token("^")]
    CARET = 138u16,
    #[token("^=")]
    CARET_EQ = 139u16,
    #[token("and")]
    AND = 140u16,
    #[token("as")]
    AS = 141u16,
    #[token("break")]
    BREAK = 142u16,
    #[regex("//.*")]
    #[regex("///.*")]
    #[token("/*", lex_block_comment)]
    COMMENT = 143u16,
    #[token("const")]
    CONST = 144u16,
    CONTINUE = 145u16,
    #[token("else")]
    ELSE = 146u16,
    #[token("enum")]
    ENUM = 147u16,
    EOF = 148u16,
    #[token("false")]
    FALSE = 149u16,
    FN = 150u16,
    #[token("for")]
    FOR = 151u16,
    #[regex("[A-Za-z_'][A-Za-z0-9_']*")]
    IDENT = 152u16,
    #[token("if")]
    IF = 153u16,
    IMPL = 154u16,
    IN = 155u16,
    #[token("let")]
    LET = 156u16,
    #[token("loop")]
    LOOP = 157u16,
    #[token("match")]
    MATCH = 158u16,
    #[regex("[0-9][0-9_]*")]
    #[regex("0b[0-1][0-1_]*")]
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*")]
    NUMBER = 159u16,
    #[token("or")]
    OR = 160u16,
    PUB = 161u16,
    #[token("return")]
    RETURN = 162u16,
    STRING = 163u16,
    #[token("struct")]
    STRUCT = 164u16,
    TOMBSTONE = 165u16,
    #[token("true")]
    TRUE = 166u16,
    TYPE = 167u16,
    USE = 168u16,
    #[token("while")]
    WHILE = 169u16,
    #[regex("[\n\t\r ]+")]
    WHITESPACE = 170u16,
    #[token("{")]
    L_CURLY = 171u16,
    #[token("|")]
    PIPE = 172u16,
    #[token("|=")]
    PIPE_EQ = 173u16,
    #[token("}")]
    R_CURLY = 174u16,
    /// An error within the syntax tree
    #[error]
    ERROR = 175u16,
}
impl SyntaxKind {
    #[doc(hidden)]
    #[inline]
    pub const fn highest() -> Self {
        Self::ERROR
    }
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
            Self::BANG => ::core::fmt::Formatter::write_str(f, "BANG"),
            Self::NEQ => ::core::fmt::Formatter::write_str(f, "NEQ"),
            Self::HASH_BRACK => ::core::fmt::Formatter::write_str(f, "HASH_BRACK"),
            Self::PERCENT => ::core::fmt::Formatter::write_str(f, "PERCENT"),
            Self::PERCENT_EQ => ::core::fmt::Formatter::write_str(f, "PERCENT_EQ"),
            Self::AMPERSAND => ::core::fmt::Formatter::write_str(f, "AMPERSAND"),
            Self::AMPERSAND_EQ => ::core::fmt::Formatter::write_str(f, "AMPERSAND_EQ"),
            Self::L_PAREN => ::core::fmt::Formatter::write_str(f, "L_PAREN"),
            Self::R_PAREN => ::core::fmt::Formatter::write_str(f, "R_PAREN"),
            Self::STAR => ::core::fmt::Formatter::write_str(f, "STAR"),
            Self::STAR_EQ => ::core::fmt::Formatter::write_str(f, "STAR_EQ"),
            Self::PLUS => ::core::fmt::Formatter::write_str(f, "PLUS"),
            Self::PLUS_EQ => ::core::fmt::Formatter::write_str(f, "PLUS_EQ"),
            Self::COMMA => ::core::fmt::Formatter::write_str(f, "COMMA"),
            Self::MINUS => ::core::fmt::Formatter::write_str(f, "MINUS"),
            Self::MINUS_EQ => ::core::fmt::Formatter::write_str(f, "MINUS_EQ"),
            Self::RIGHT_ARROW => ::core::fmt::Formatter::write_str(f, "RIGHT_ARROW"),
            Self::DOT => ::core::fmt::Formatter::write_str(f, "DOT"),
            Self::SLASH => ::core::fmt::Formatter::write_str(f, "SLASH"),
            Self::SLASH_EQ => ::core::fmt::Formatter::write_str(f, "SLASH_EQ"),
            Self::COLON => ::core::fmt::Formatter::write_str(f, "COLON"),
            Self::DOUBLE_COLON => ::core::fmt::Formatter::write_str(f, "DOUBLE_COLON"),
            Self::SEMICOLON => ::core::fmt::Formatter::write_str(f, "SEMICOLON"),
            Self::L_ANGLE => ::core::fmt::Formatter::write_str(f, "L_ANGLE"),
            Self::SHL => ::core::fmt::Formatter::write_str(f, "SHL"),
            Self::SHL_EQ => ::core::fmt::Formatter::write_str(f, "SHL_EQ"),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "L_ANGLE_EQ"),
            Self::EQ => ::core::fmt::Formatter::write_str(f, "EQ"),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "EQEQ"),
            Self::RIGHT_ROCKET => ::core::fmt::Formatter::write_str(f, "RIGHT_ROCKET"),
            Self::R_ANGLE => ::core::fmt::Formatter::write_str(f, "R_ANGLE"),
            Self::R_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "R_ANGLE_EQ"),
            Self::SHR => ::core::fmt::Formatter::write_str(f, "SHR"),
            Self::SHR_EQ => ::core::fmt::Formatter::write_str(f, "SHR_EQ"),
            Self::ARRAY_ACCESS => ::core::fmt::Formatter::write_str(f, "ARRAY_ACCESS"),
            Self::ARRAY_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "ARRAY_EXPR_ELEM"),
            Self::ARRAY_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "ARRAY_INIT_EXPR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ASSIGN_OP => ::core::fmt::Formatter::write_str(f, "ASSIGN_OP"),
            Self::ATTR_NAME => ::core::fmt::Formatter::write_str(f, "ATTR_NAME"),
            Self::ATTR_PAIR => ::core::fmt::Formatter::write_str(f, "ATTR_PAIR"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
            Self::BIN_EXPR => ::core::fmt::Formatter::write_str(f, "BIN_EXPR"),
            Self::BIN_OP => ::core::fmt::Formatter::write_str(f, "BIN_OP"),
            Self::BLOCK => ::core::fmt::Formatter::write_str(f, "BLOCK"),
            Self::BOOL => ::core::fmt::Formatter::write_str(f, "BOOL"),
            Self::BRACKED_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "BRACKED_STRUCT_FIELD")
            }
            Self::BRACKED_STRUCT_FIELDS => {
                ::core::fmt::Formatter::write_str(f, "BRACKED_STRUCT_FIELDS")
            }
            Self::BREAK_EXPR => ::core::fmt::Formatter::write_str(f, "BREAK_EXPR"),
            Self::CLOSURE_ARG => ::core::fmt::Formatter::write_str(f, "CLOSURE_ARG"),
            Self::CLOSURE_EXPR => ::core::fmt::Formatter::write_str(f, "CLOSURE_EXPR"),
            Self::CONST_DEF => ::core::fmt::Formatter::write_str(f, "CONST_DEF"),
            Self::CONST_NAME => ::core::fmt::Formatter::write_str(f, "CONST_NAME"),
            Self::CONST_VALUE => ::core::fmt::Formatter::write_str(f, "CONST_VALUE"),
            Self::CONTINUE_EXPR => ::core::fmt::Formatter::write_str(f, "CONTINUE_EXPR"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::ENUM_DEF => ::core::fmt::Formatter::write_str(f, "ENUM_DEF"),
            Self::ENUM_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_NAME"),
            Self::ENUM_VARIANT => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT"),
            Self::ENUM_VARIANT_BODY => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_BODY"),
            Self::ENUM_VARIANT_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_NAME"),
            Self::ENUM_VARIANTS => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANTS"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR"),
            Self::FIELD_ACCESSOR_NAME => {
                ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR_NAME")
            }
            Self::FOR_EXPR => ::core::fmt::Formatter::write_str(f, "FOR_EXPR"),
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
            Self::GENERIC_ARG => ::core::fmt::Formatter::write_str(f, "GENERIC_ARG"),
            Self::GENERIC_TYPE => ::core::fmt::Formatter::write_str(f, "GENERIC_TYPE"),
            Self::GENERICS => ::core::fmt::Formatter::write_str(f, "GENERICS"),
            Self::IF_BLOCK => ::core::fmt::Formatter::write_str(f, "IF_BLOCK"),
            Self::IF_EXPR => ::core::fmt::Formatter::write_str(f, "IF_EXPR"),
            Self::IMPL_BLOCK => ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK"),
            Self::IMPL_BLOCK_CONTENTS => {
                ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK_CONTENTS")
            }
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
            Self::LOOP_EXPR => ::core::fmt::Formatter::write_str(f, "LOOP_EXPR"),
            Self::MATCH_ARM => ::core::fmt::Formatter::write_str(f, "MATCH_ARM"),
            Self::MATCH_EXPR => ::core::fmt::Formatter::write_str(f, "MATCH_EXPR"),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATH => ::core::fmt::Formatter::write_str(f, "PATH"),
            Self::PATH_SEGMENT => ::core::fmt::Formatter::write_str(f, "PATH_SEGMENT"),
            Self::PATH_TAIL => ::core::fmt::Formatter::write_str(f, "PATH_TAIL"),
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
            Self::QUALIFIED_REF => ::core::fmt::Formatter::write_str(f, "QUALIFIED_REF"),
            Self::RET_EXPR => ::core::fmt::Formatter::write_str(f, "RET_EXPR"),
            Self::ROOT => ::core::fmt::Formatter::write_str(f, "ROOT"),
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_FIELD_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELD_NAME"),
            Self::STRUCT_FIELDS => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELDS"),
            Self::STRUCT_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_EXPR"),
            Self::STRUCT_INIT_FIELD => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_FIELD"),
            Self::STRUCT_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_NAME"),
            Self::STRUCT_PATTERN => ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN"),
            Self::STRUCT_PATTERN_FIELD => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD")
            }
            Self::STRUCT_PATTERN_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD_NAME")
            }
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
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_ALIAS_NAME => ::core::fmt::Formatter::write_str(f, "USE_ALIAS_NAME"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_BRANCH_OR_ALIAS => {
                ::core::fmt::Formatter::write_str(f, "USE_BRANCH_OR_ALIAS")
            }
            Self::USE_DEF => ::core::fmt::Formatter::write_str(f, "USE_DEF"),
            Self::USE_TREE => ::core::fmt::Formatter::write_str(f, "USE_TREE"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::VARIANT_STRUCT => ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT"),
            Self::VARIANT_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD")
            }
            Self::VARIANT_STRUCT_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD_NAME")
            }
            Self::VARIANT_TUPLE => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE"),
            Self::VARIANT_TUPLE_ELEM => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE_ELEM"),
            Self::WHILE_EXPR => ::core::fmt::Formatter::write_str(f, "WHILE_EXPR"),
            Self::L_BRACK => ::core::fmt::Formatter::write_str(f, "L_BRACK"),
            Self::R_BRACK => ::core::fmt::Formatter::write_str(f, "R_BRACK"),
            Self::CARET => ::core::fmt::Formatter::write_str(f, "CARET"),
            Self::CARET_EQ => ::core::fmt::Formatter::write_str(f, "CARET_EQ"),
            Self::AND => ::core::fmt::Formatter::write_str(f, "AND"),
            Self::AS => ::core::fmt::Formatter::write_str(f, "AS"),
            Self::BREAK => ::core::fmt::Formatter::write_str(f, "BREAK"),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::CONST => ::core::fmt::Formatter::write_str(f, "CONST"),
            Self::CONTINUE => ::core::fmt::Formatter::write_str(f, "CONTINUE"),
            Self::ELSE => ::core::fmt::Formatter::write_str(f, "ELSE"),
            Self::ENUM => ::core::fmt::Formatter::write_str(f, "ENUM"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::FALSE => ::core::fmt::Formatter::write_str(f, "FALSE"),
            Self::FN => ::core::fmt::Formatter::write_str(f, "FN"),
            Self::FOR => ::core::fmt::Formatter::write_str(f, "FOR"),
            Self::IDENT => ::core::fmt::Formatter::write_str(f, "IDENT"),
            Self::IF => ::core::fmt::Formatter::write_str(f, "IF"),
            Self::IMPL => ::core::fmt::Formatter::write_str(f, "IMPL"),
            Self::IN => ::core::fmt::Formatter::write_str(f, "IN"),
            Self::LET => ::core::fmt::Formatter::write_str(f, "LET"),
            Self::LOOP => ::core::fmt::Formatter::write_str(f, "LOOP"),
            Self::MATCH => ::core::fmt::Formatter::write_str(f, "MATCH"),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::OR => ::core::fmt::Formatter::write_str(f, "OR"),
            Self::PUB => ::core::fmt::Formatter::write_str(f, "PUB"),
            Self::RETURN => ::core::fmt::Formatter::write_str(f, "RETURN"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRUCT => ::core::fmt::Formatter::write_str(f, "STRUCT"),
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE => ::core::fmt::Formatter::write_str(f, "TRUE"),
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::USE => ::core::fmt::Formatter::write_str(f, "USE"),
            Self::WHILE => ::core::fmt::Formatter::write_str(f, "WHILE"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
            Self::L_CURLY => ::core::fmt::Formatter::write_str(f, "L_CURLY"),
            Self::PIPE => ::core::fmt::Formatter::write_str(f, "PIPE"),
            Self::PIPE_EQ => ::core::fmt::Formatter::write_str(f, "PIPE_EQ"),
            Self::R_CURLY => ::core::fmt::Formatter::write_str(f, "R_CURLY"),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "ERROR"),
        }
    }
}
impl ::core::fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::BANG => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '!'),
            Self::NEQ => ::core::fmt::Formatter::write_str(f, "!="),
            Self::HASH_BRACK => ::core::fmt::Formatter::write_str(f, "#["),
            Self::PERCENT => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '%'),
            Self::PERCENT_EQ => ::core::fmt::Formatter::write_str(f, "%="),
            Self::AMPERSAND => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '&'),
            Self::AMPERSAND_EQ => ::core::fmt::Formatter::write_str(f, "&="),
            Self::L_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '('),
            Self::R_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ')'),
            Self::STAR => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '*'),
            Self::STAR_EQ => ::core::fmt::Formatter::write_str(f, "*="),
            Self::PLUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '+'),
            Self::PLUS_EQ => ::core::fmt::Formatter::write_str(f, "+="),
            Self::COMMA => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ','),
            Self::MINUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '-'),
            Self::MINUS_EQ => ::core::fmt::Formatter::write_str(f, "-="),
            Self::RIGHT_ARROW => ::core::fmt::Formatter::write_str(f, "->"),
            Self::DOT => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '.'),
            Self::SLASH => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '/'),
            Self::SLASH_EQ => ::core::fmt::Formatter::write_str(f, "/="),
            Self::COLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ':'),
            Self::DOUBLE_COLON => ::core::fmt::Formatter::write_str(f, "::"),
            Self::SEMICOLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ';'),
            Self::L_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '<'),
            Self::SHL => ::core::fmt::Formatter::write_str(f, "<<"),
            Self::SHL_EQ => ::core::fmt::Formatter::write_str(f, "<<="),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "<="),
            Self::EQ => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '='),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "=="),
            Self::RIGHT_ROCKET => ::core::fmt::Formatter::write_str(f, "=>"),
            Self::R_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '>'),
            Self::R_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, ">="),
            Self::SHR => ::core::fmt::Formatter::write_str(f, ">>"),
            Self::SHR_EQ => ::core::fmt::Formatter::write_str(f, ">>="),
            Self::ARRAY_ACCESS => ::core::fmt::Formatter::write_str(f, "ARRAY_ACCESS"),
            Self::ARRAY_EXPR_ELEM => ::core::fmt::Formatter::write_str(f, "ARRAY_EXPR_ELEM"),
            Self::ARRAY_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "ARRAY_INIT_EXPR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ASSIGN_OP => ::core::fmt::Formatter::write_str(f, "ASSIGN_OP"),
            Self::ATTR_NAME => ::core::fmt::Formatter::write_str(f, "ATTR_NAME"),
            Self::ATTR_PAIR => ::core::fmt::Formatter::write_str(f, "ATTR_PAIR"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
            Self::BIN_EXPR => ::core::fmt::Formatter::write_str(f, "BIN_EXPR"),
            Self::BIN_OP => ::core::fmt::Formatter::write_str(f, "BIN_OP"),
            Self::BLOCK => ::core::fmt::Formatter::write_str(f, "BLOCK"),
            Self::BOOL => ::core::fmt::Formatter::write_str(f, "BOOL"),
            Self::BRACKED_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "BRACKED_STRUCT_FIELD")
            }
            Self::BRACKED_STRUCT_FIELDS => {
                ::core::fmt::Formatter::write_str(f, "BRACKED_STRUCT_FIELDS")
            }
            Self::BREAK_EXPR => ::core::fmt::Formatter::write_str(f, "BREAK_EXPR"),
            Self::CLOSURE_ARG => ::core::fmt::Formatter::write_str(f, "CLOSURE_ARG"),
            Self::CLOSURE_EXPR => ::core::fmt::Formatter::write_str(f, "CLOSURE_EXPR"),
            Self::CONST_DEF => ::core::fmt::Formatter::write_str(f, "CONST_DEF"),
            Self::CONST_NAME => ::core::fmt::Formatter::write_str(f, "CONST_NAME"),
            Self::CONST_VALUE => ::core::fmt::Formatter::write_str(f, "CONST_VALUE"),
            Self::CONTINUE_EXPR => ::core::fmt::Formatter::write_str(f, "CONTINUE_EXPR"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::ENUM_DEF => ::core::fmt::Formatter::write_str(f, "ENUM_DEF"),
            Self::ENUM_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_NAME"),
            Self::ENUM_VARIANT => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT"),
            Self::ENUM_VARIANT_BODY => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_BODY"),
            Self::ENUM_VARIANT_NAME => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANT_NAME"),
            Self::ENUM_VARIANTS => ::core::fmt::Formatter::write_str(f, "ENUM_VARIANTS"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FIELD_ACCESS => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESS"),
            Self::FIELD_ACCESSOR => ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR"),
            Self::FIELD_ACCESSOR_NAME => {
                ::core::fmt::Formatter::write_str(f, "FIELD_ACCESSOR_NAME")
            }
            Self::FOR_EXPR => ::core::fmt::Formatter::write_str(f, "FOR_EXPR"),
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
            Self::GENERIC_ARG => ::core::fmt::Formatter::write_str(f, "GENERIC_ARG"),
            Self::GENERIC_TYPE => ::core::fmt::Formatter::write_str(f, "GENERIC_TYPE"),
            Self::GENERICS => ::core::fmt::Formatter::write_str(f, "GENERICS"),
            Self::IF_BLOCK => ::core::fmt::Formatter::write_str(f, "IF_BLOCK"),
            Self::IF_EXPR => ::core::fmt::Formatter::write_str(f, "IF_EXPR"),
            Self::IMPL_BLOCK => ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK"),
            Self::IMPL_BLOCK_CONTENTS => {
                ::core::fmt::Formatter::write_str(f, "IMPL_BLOCK_CONTENTS")
            }
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
            Self::LOOP_EXPR => ::core::fmt::Formatter::write_str(f, "LOOP_EXPR"),
            Self::MATCH_ARM => ::core::fmt::Formatter::write_str(f, "MATCH_ARM"),
            Self::MATCH_EXPR => ::core::fmt::Formatter::write_str(f, "MATCH_EXPR"),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATH => ::core::fmt::Formatter::write_str(f, "PATH"),
            Self::PATH_SEGMENT => ::core::fmt::Formatter::write_str(f, "PATH_SEGMENT"),
            Self::PATH_TAIL => ::core::fmt::Formatter::write_str(f, "PATH_TAIL"),
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
            Self::QUALIFIED_REF => ::core::fmt::Formatter::write_str(f, "QUALIFIED_REF"),
            Self::RET_EXPR => ::core::fmt::Formatter::write_str(f, "RET_EXPR"),
            Self::ROOT => ::core::fmt::Formatter::write_str(f, "ROOT"),
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::STRUCT_DEF => ::core::fmt::Formatter::write_str(f, "STRUCT_DEF"),
            Self::STRUCT_FIELD_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELD_NAME"),
            Self::STRUCT_FIELDS => ::core::fmt::Formatter::write_str(f, "STRUCT_FIELDS"),
            Self::STRUCT_INIT_EXPR => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_EXPR"),
            Self::STRUCT_INIT_FIELD => ::core::fmt::Formatter::write_str(f, "STRUCT_INIT_FIELD"),
            Self::STRUCT_NAME => ::core::fmt::Formatter::write_str(f, "STRUCT_NAME"),
            Self::STRUCT_PATTERN => ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN"),
            Self::STRUCT_PATTERN_FIELD => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD")
            }
            Self::STRUCT_PATTERN_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "STRUCT_PATTERN_FIELD_NAME")
            }
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
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::USE_ALIAS => ::core::fmt::Formatter::write_str(f, "USE_ALIAS"),
            Self::USE_ALIAS_NAME => ::core::fmt::Formatter::write_str(f, "USE_ALIAS_NAME"),
            Self::USE_BRANCH => ::core::fmt::Formatter::write_str(f, "USE_BRANCH"),
            Self::USE_BRANCH_OR_ALIAS => {
                ::core::fmt::Formatter::write_str(f, "USE_BRANCH_OR_ALIAS")
            }
            Self::USE_DEF => ::core::fmt::Formatter::write_str(f, "USE_DEF"),
            Self::USE_TREE => ::core::fmt::Formatter::write_str(f, "USE_TREE"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::VARIANT_STRUCT => ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT"),
            Self::VARIANT_STRUCT_FIELD => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD")
            }
            Self::VARIANT_STRUCT_FIELD_NAME => {
                ::core::fmt::Formatter::write_str(f, "VARIANT_STRUCT_FIELD_NAME")
            }
            Self::VARIANT_TUPLE => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE"),
            Self::VARIANT_TUPLE_ELEM => ::core::fmt::Formatter::write_str(f, "VARIANT_TUPLE_ELEM"),
            Self::WHILE_EXPR => ::core::fmt::Formatter::write_str(f, "WHILE_EXPR"),
            Self::L_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '['),
            Self::R_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ']'),
            Self::CARET => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '^'),
            Self::CARET_EQ => ::core::fmt::Formatter::write_str(f, "^="),
            Self::AND => ::core::fmt::Formatter::write_str(f, "and"),
            Self::AS => ::core::fmt::Formatter::write_str(f, "as"),
            Self::BREAK => ::core::fmt::Formatter::write_str(f, "break"),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::CONST => ::core::fmt::Formatter::write_str(f, "const"),
            Self::CONTINUE => ::core::fmt::Formatter::write_str(f, "CONTINUE"),
            Self::ELSE => ::core::fmt::Formatter::write_str(f, "else"),
            Self::ENUM => ::core::fmt::Formatter::write_str(f, "enum"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::FALSE => ::core::fmt::Formatter::write_str(f, "false"),
            Self::FN => ::core::fmt::Formatter::write_str(f, "FN"),
            Self::FOR => ::core::fmt::Formatter::write_str(f, "for"),
            Self::IDENT => ::core::fmt::Formatter::write_str(f, "IDENT"),
            Self::IF => ::core::fmt::Formatter::write_str(f, "if"),
            Self::IMPL => ::core::fmt::Formatter::write_str(f, "IMPL"),
            Self::IN => ::core::fmt::Formatter::write_str(f, "IN"),
            Self::LET => ::core::fmt::Formatter::write_str(f, "let"),
            Self::LOOP => ::core::fmt::Formatter::write_str(f, "loop"),
            Self::MATCH => ::core::fmt::Formatter::write_str(f, "match"),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::OR => ::core::fmt::Formatter::write_str(f, "or"),
            Self::PUB => ::core::fmt::Formatter::write_str(f, "PUB"),
            Self::RETURN => ::core::fmt::Formatter::write_str(f, "return"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::STRUCT => ::core::fmt::Formatter::write_str(f, "struct"),
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE => ::core::fmt::Formatter::write_str(f, "true"),
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::USE => ::core::fmt::Formatter::write_str(f, "USE"),
            Self::WHILE => ::core::fmt::Formatter::write_str(f, "while"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
            Self::L_CURLY => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '{'),
            Self::PIPE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '|'),
            Self::PIPE_EQ => ::core::fmt::Formatter::write_str(f, "|="),
            Self::R_CURLY => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '}'),
            Self::ERROR => ::core::fmt::Formatter::write_str(f, "???"),
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
            "invalid SyntaxKind '{}', must be within the range of 0..={}",
            kind,
            SyntaxKind::ERROR as u16,
        )
    }
    impl ::core::convert::From<u16> for SyntaxKind {
        #[inline]
        #[track_caller]
        fn from(kind: u16) -> Self {
            if kind > Self::ERROR as u16 {
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
            if kind > Self::ERROR as u16 {
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
#[macro_export]
macro_rules! T {
    (!) => {
        $crate::SyntaxKind::BANG
    };
    (!=) => {
        $crate::SyntaxKind::NEQ
    };
    ("#[") => {
        $crate::SyntaxKind::HASH_BRACK
    };
    (%) => {
        $crate::SyntaxKind::PERCENT
    };
    (%=) => {
        $crate::SyntaxKind::PERCENT_EQ
    };
    (&) => {
        $crate::SyntaxKind::AMPERSAND
    };
    (&=) => {
        $crate::SyntaxKind::AMPERSAND_EQ
    };
    ('(') => {
        $crate::SyntaxKind::L_PAREN
    };
    (')') => {
        $crate::SyntaxKind::R_PAREN
    };
    (*) => {
        $crate::SyntaxKind::STAR
    };
    (*=) => {
        $crate::SyntaxKind::STAR_EQ
    };
    (+) => {
        $crate::SyntaxKind::PLUS
    };
    (+=) => {
        $crate::SyntaxKind::PLUS_EQ
    };
    (,) => {
        $crate::SyntaxKind::COMMA
    };
    (-) => {
        $crate::SyntaxKind::MINUS
    };
    (-=) => {
        $crate::SyntaxKind::MINUS_EQ
    };
    (->) => {
        $crate::SyntaxKind::RIGHT_ARROW
    };
    (.) => {
        $crate::SyntaxKind::DOT
    };
    (/) => {
        $crate::SyntaxKind::SLASH
    };
    (/=) => {
        $crate::SyntaxKind::SLASH_EQ
    };
    (:) => {
        $crate::SyntaxKind::COLON
    };
    (::) => {
        $crate::SyntaxKind::DOUBLE_COLON
    };
    (;) => {
        $crate::SyntaxKind::SEMICOLON
    };
    (<) => {
        $crate::SyntaxKind::L_ANGLE
    };
    (<<) => {
        $crate::SyntaxKind::SHL
    };
    (<<=) => {
        $crate::SyntaxKind::SHL_EQ
    };
    (<=) => {
        $crate::SyntaxKind::L_ANGLE_EQ
    };
    (=) => {
        $crate::SyntaxKind::EQ
    };
    (==) => {
        $crate::SyntaxKind::EQEQ
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
    (>>) => {
        $crate::SyntaxKind::SHR
    };
    (>>=) => {
        $crate::SyntaxKind::SHR_EQ
    };
    ('[') => {
        $crate::SyntaxKind::L_BRACK
    };
    (']') => {
        $crate::SyntaxKind::R_BRACK
    };
    (^) => {
        $crate::SyntaxKind::CARET
    };
    (^=) => {
        $crate::SyntaxKind::CARET_EQ
    };
    (and) => {
        $crate::SyntaxKind::AND
    };
    (as) => {
        $crate::SyntaxKind::AS
    };
    (break) => {
        $crate::SyntaxKind::BREAK
    };
    (const) => {
        $crate::SyntaxKind::CONST
    };
    (else) => {
        $crate::SyntaxKind::ELSE
    };
    (enum) => {
        $crate::SyntaxKind::ENUM
    };
    (false) => {
        $crate::SyntaxKind::FALSE
    };
    (for) => {
        $crate::SyntaxKind::FOR
    };
    (if) => {
        $crate::SyntaxKind::IF
    };
    (let) => {
        $crate::SyntaxKind::LET
    };
    (loop) => {
        $crate::SyntaxKind::LOOP
    };
    (match) => {
        $crate::SyntaxKind::MATCH
    };
    (or) => {
        $crate::SyntaxKind::OR
    };
    (return) => {
        $crate::SyntaxKind::RETURN
    };
    (struct) => {
        $crate::SyntaxKind::STRUCT
    };
    (true) => {
        $crate::SyntaxKind::TRUE
    };
    (while) => {
        $crate::SyntaxKind::WHILE
    };
    ('{') => {
        $crate::SyntaxKind::L_CURLY
    };
    (|) => {
        $crate::SyntaxKind::PIPE
    };
    (|=) => {
        $crate::SyntaxKind::PIPE_EQ
    };
    ('}') => {
        $crate::SyntaxKind::R_CURLY
    };
}
