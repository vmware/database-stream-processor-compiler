#[derive(logos :: Logos)]
#[allow(
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::manual_non_exhaustive
)]
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
    #[token("&")]
    AMPERSAND = 4u16,
    #[token("(")]
    L_PAREN = 5u16,
    #[token(")")]
    R_PAREN = 6u16,
    #[token("*")]
    STAR = 7u16,
    #[token("+")]
    PLUS = 8u16,
    #[token(",")]
    COMMA = 9u16,
    #[token("-")]
    MINUS = 10u16,
    #[token("/")]
    SLASH = 11u16,
    #[token(":")]
    COLON = 12u16,
    #[token(";")]
    SEMICOLON = 13u16,
    #[token("<")]
    L_ANGLE = 14u16,
    #[token("<<")]
    SHL = 15u16,
    #[token("<=")]
    L_ANGLE_EQ = 16u16,
    #[token("=")]
    EQ = 17u16,
    #[token("==")]
    EQEQ = 18u16,
    #[token("=>")]
    RIGHT_ROCKET = 19u16,
    #[token(">")]
    R_ANGLE = 20u16,
    #[token(">=")]
    R_ANGLE_EQ = 21u16,
    #[token(">>")]
    SHR = 22u16,
    ASSIGN = 23u16,
    ATTR_PAIR = 24u16,
    ATTRIBUTE = 25u16,
    ATTRIBUTES = 26u16,
    BIN_EXPR = 27u16,
    BIN_OP = 28u16,
    BLOCK = 29u16,
    BOOL = 30u16,
    ELSE_BLOCK = 31u16,
    EQ_EQ = 32u16,
    EXPR = 33u16,
    EXPR_STMT = 34u16,
    FUNCTION_ARG = 35u16,
    FUNCTION_ARGS = 36u16,
    FUNCTION_DEF = 37u16,
    FUNCTION_NAME = 38u16,
    FUNCTION_RETURN = 39u16,
    FUNCTION_RETURN_TYPE = 40u16,
    FUNCTION_TYPE = 41u16,
    FUNCTION_TYPE_ARG = 42u16,
    FUNCTION_TYPE_ARGS = 43u16,
    GENERIC_ARG = 44u16,
    GENERIC_TYPE = 45u16,
    GENERICS = 46u16,
    IF_BLOCK = 47u16,
    IF_STMT = 48u16,
    ITEM = 49u16,
    LITERAL = 50u16,
    MODIFIER = 51u16,
    MODIFIERS = 52u16,
    PAREN_EXPR = 53u16,
    PATTERN = 54u16,
    RECORD_FIELD = 55u16,
    RECORD_NAME = 56u16,
    RECORD_TYPE = 57u16,
    REL_COL = 58u16,
    REL_COLS = 59u16,
    REL_KW = 60u16,
    REL_NAME = 61u16,
    RELATION_DEF = 62u16,
    RET_EXPR = 63u16,
    ROOT = 64u16,
    STMT = 65u16,
    SUM_TYPE = 66u16,
    TUPLE_PATTERN = 67u16,
    TUPLE_PATTERN_ELEM = 68u16,
    TUPLE_TYPE = 69u16,
    TUPLE_TYPE_ELEM = 70u16,
    TYPE = 71u16,
    TYPE_BODY = 72u16,
    TYPE_DEF = 73u16,
    TYPE_NAME = 74u16,
    UNARY_EXPR = 75u16,
    UNARY_OP = 76u16,
    VAR_DECL = 77u16,
    VAR_REF = 78u16,
    #[token("[")]
    L_BRACK = 79u16,
    #[token("]")]
    R_BRACK = 80u16,
    #[token("^")]
    CARET = 81u16,
    #[token("and")]
    AND = 82u16,
    #[regex("//.*")]
    #[regex("///.*")]
    #[token("/*", lex_block_comment)]
    COMMENT = 83u16,
    #[token("else")]
    ELSE = 84u16,
    EOF = 85u16,
    #[token("extern")]
    EXTERN = 86u16,
    #[token("false")]
    FALSE = 87u16,
    #[token("function")]
    FUNCTION = 88u16,
    #[regex("[A-Za-z_'][A-Za-z0-9_']*")]
    IDENT = 89u16,
    #[token("if")]
    IF = 90u16,
    #[token("input")]
    INPUT = 91u16,
    #[token("multiset")]
    MULTISET = 92u16,
    #[regex("[0-9][0-9_]*")]
    #[regex("0b[0-1][0-1_]*")]
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*")]
    NUMBER = 93u16,
    #[token("or")]
    OR = 94u16,
    #[token("output")]
    OUTPUT = 95u16,
    #[token("relation")]
    RELATION = 96u16,
    #[token("return")]
    RETURN = 97u16,
    #[token("stream")]
    STREAM = 98u16,
    STRING = 99u16,
    TOMBSTONE = 100u16,
    #[token("true")]
    TRUE = 101u16,
    #[token("typedef")]
    TYPEDEF = 102u16,
    #[token("var")]
    VAR = 103u16,
    #[regex("[\n\t\r ]+")]
    WHITESPACE = 104u16,
    #[token("{")]
    L_CURLY = 105u16,
    #[token("|")]
    PIPE = 106u16,
    #[token("}")]
    R_CURLY = 107u16,
    /// An error within the syntax tree
    #[error]
    ERROR = 108u16,
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
            Self::AMPERSAND => ::core::fmt::Formatter::write_str(f, "AMPERSAND"),
            Self::L_PAREN => ::core::fmt::Formatter::write_str(f, "L_PAREN"),
            Self::R_PAREN => ::core::fmt::Formatter::write_str(f, "R_PAREN"),
            Self::STAR => ::core::fmt::Formatter::write_str(f, "STAR"),
            Self::PLUS => ::core::fmt::Formatter::write_str(f, "PLUS"),
            Self::COMMA => ::core::fmt::Formatter::write_str(f, "COMMA"),
            Self::MINUS => ::core::fmt::Formatter::write_str(f, "MINUS"),
            Self::SLASH => ::core::fmt::Formatter::write_str(f, "SLASH"),
            Self::COLON => ::core::fmt::Formatter::write_str(f, "COLON"),
            Self::SEMICOLON => ::core::fmt::Formatter::write_str(f, "SEMICOLON"),
            Self::L_ANGLE => ::core::fmt::Formatter::write_str(f, "L_ANGLE"),
            Self::SHL => ::core::fmt::Formatter::write_str(f, "SHL"),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "L_ANGLE_EQ"),
            Self::EQ => ::core::fmt::Formatter::write_str(f, "EQ"),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "EQEQ"),
            Self::RIGHT_ROCKET => ::core::fmt::Formatter::write_str(f, "RIGHT_ROCKET"),
            Self::R_ANGLE => ::core::fmt::Formatter::write_str(f, "R_ANGLE"),
            Self::R_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "R_ANGLE_EQ"),
            Self::SHR => ::core::fmt::Formatter::write_str(f, "SHR"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ATTR_PAIR => ::core::fmt::Formatter::write_str(f, "ATTR_PAIR"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
            Self::ATTRIBUTES => ::core::fmt::Formatter::write_str(f, "ATTRIBUTES"),
            Self::BIN_EXPR => ::core::fmt::Formatter::write_str(f, "BIN_EXPR"),
            Self::BIN_OP => ::core::fmt::Formatter::write_str(f, "BIN_OP"),
            Self::BLOCK => ::core::fmt::Formatter::write_str(f, "BLOCK"),
            Self::BOOL => ::core::fmt::Formatter::write_str(f, "BOOL"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::EQ_EQ => ::core::fmt::Formatter::write_str(f, "EQ_EQ"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FUNCTION_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARG"),
            Self::FUNCTION_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARGS"),
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
            Self::IF_STMT => ::core::fmt::Formatter::write_str(f, "IF_STMT"),
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::MODIFIERS => ::core::fmt::Formatter::write_str(f, "MODIFIERS"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
            Self::RECORD_FIELD => ::core::fmt::Formatter::write_str(f, "RECORD_FIELD"),
            Self::RECORD_NAME => ::core::fmt::Formatter::write_str(f, "RECORD_NAME"),
            Self::RECORD_TYPE => ::core::fmt::Formatter::write_str(f, "RECORD_TYPE"),
            Self::REL_COL => ::core::fmt::Formatter::write_str(f, "REL_COL"),
            Self::REL_COLS => ::core::fmt::Formatter::write_str(f, "REL_COLS"),
            Self::REL_KW => ::core::fmt::Formatter::write_str(f, "REL_KW"),
            Self::REL_NAME => ::core::fmt::Formatter::write_str(f, "REL_NAME"),
            Self::RELATION_DEF => ::core::fmt::Formatter::write_str(f, "RELATION_DEF"),
            Self::RET_EXPR => ::core::fmt::Formatter::write_str(f, "RET_EXPR"),
            Self::ROOT => ::core::fmt::Formatter::write_str(f, "ROOT"),
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::SUM_TYPE => ::core::fmt::Formatter::write_str(f, "SUM_TYPE"),
            Self::TUPLE_PATTERN => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN"),
            Self::TUPLE_PATTERN_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN_ELEM"),
            Self::TUPLE_TYPE => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE"),
            Self::TUPLE_TYPE_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE_ELEM"),
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::TYPE_BODY => ::core::fmt::Formatter::write_str(f, "TYPE_BODY"),
            Self::TYPE_DEF => ::core::fmt::Formatter::write_str(f, "TYPE_DEF"),
            Self::TYPE_NAME => ::core::fmt::Formatter::write_str(f, "TYPE_NAME"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::L_BRACK => ::core::fmt::Formatter::write_str(f, "L_BRACK"),
            Self::R_BRACK => ::core::fmt::Formatter::write_str(f, "R_BRACK"),
            Self::CARET => ::core::fmt::Formatter::write_str(f, "CARET"),
            Self::AND => ::core::fmt::Formatter::write_str(f, "AND"),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::ELSE => ::core::fmt::Formatter::write_str(f, "ELSE"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EXTERN => ::core::fmt::Formatter::write_str(f, "EXTERN"),
            Self::FALSE => ::core::fmt::Formatter::write_str(f, "FALSE"),
            Self::FUNCTION => ::core::fmt::Formatter::write_str(f, "FUNCTION"),
            Self::IDENT => ::core::fmt::Formatter::write_str(f, "IDENT"),
            Self::IF => ::core::fmt::Formatter::write_str(f, "IF"),
            Self::INPUT => ::core::fmt::Formatter::write_str(f, "INPUT"),
            Self::MULTISET => ::core::fmt::Formatter::write_str(f, "MULTISET"),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::OR => ::core::fmt::Formatter::write_str(f, "OR"),
            Self::OUTPUT => ::core::fmt::Formatter::write_str(f, "OUTPUT"),
            Self::RELATION => ::core::fmt::Formatter::write_str(f, "RELATION"),
            Self::RETURN => ::core::fmt::Formatter::write_str(f, "RETURN"),
            Self::STREAM => ::core::fmt::Formatter::write_str(f, "STREAM"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE => ::core::fmt::Formatter::write_str(f, "TRUE"),
            Self::TYPEDEF => ::core::fmt::Formatter::write_str(f, "TYPEDEF"),
            Self::VAR => ::core::fmt::Formatter::write_str(f, "VAR"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
            Self::L_CURLY => ::core::fmt::Formatter::write_str(f, "L_CURLY"),
            Self::PIPE => ::core::fmt::Formatter::write_str(f, "PIPE"),
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
            Self::AMPERSAND => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '&'),
            Self::L_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '('),
            Self::R_PAREN => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ')'),
            Self::STAR => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '*'),
            Self::PLUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '+'),
            Self::COMMA => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ','),
            Self::MINUS => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '-'),
            Self::SLASH => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '/'),
            Self::COLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ':'),
            Self::SEMICOLON => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ';'),
            Self::L_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '<'),
            Self::SHL => ::core::fmt::Formatter::write_str(f, "<<"),
            Self::L_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, "<="),
            Self::EQ => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '='),
            Self::EQEQ => ::core::fmt::Formatter::write_str(f, "=="),
            Self::RIGHT_ROCKET => ::core::fmt::Formatter::write_str(f, "=>"),
            Self::R_ANGLE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '>'),
            Self::R_ANGLE_EQ => ::core::fmt::Formatter::write_str(f, ">="),
            Self::SHR => ::core::fmt::Formatter::write_str(f, ">>"),
            Self::ASSIGN => ::core::fmt::Formatter::write_str(f, "ASSIGN"),
            Self::ATTR_PAIR => ::core::fmt::Formatter::write_str(f, "ATTR_PAIR"),
            Self::ATTRIBUTE => ::core::fmt::Formatter::write_str(f, "ATTRIBUTE"),
            Self::ATTRIBUTES => ::core::fmt::Formatter::write_str(f, "ATTRIBUTES"),
            Self::BIN_EXPR => ::core::fmt::Formatter::write_str(f, "BIN_EXPR"),
            Self::BIN_OP => ::core::fmt::Formatter::write_str(f, "BIN_OP"),
            Self::BLOCK => ::core::fmt::Formatter::write_str(f, "BLOCK"),
            Self::BOOL => ::core::fmt::Formatter::write_str(f, "BOOL"),
            Self::ELSE_BLOCK => ::core::fmt::Formatter::write_str(f, "ELSE_BLOCK"),
            Self::EQ_EQ => ::core::fmt::Formatter::write_str(f, "EQ_EQ"),
            Self::EXPR => ::core::fmt::Formatter::write_str(f, "EXPR"),
            Self::EXPR_STMT => ::core::fmt::Formatter::write_str(f, "EXPR_STMT"),
            Self::FUNCTION_ARG => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARG"),
            Self::FUNCTION_ARGS => ::core::fmt::Formatter::write_str(f, "FUNCTION_ARGS"),
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
            Self::IF_STMT => ::core::fmt::Formatter::write_str(f, "IF_STMT"),
            Self::ITEM => ::core::fmt::Formatter::write_str(f, "ITEM"),
            Self::LITERAL => ::core::fmt::Formatter::write_str(f, "LITERAL"),
            Self::MODIFIER => ::core::fmt::Formatter::write_str(f, "MODIFIER"),
            Self::MODIFIERS => ::core::fmt::Formatter::write_str(f, "MODIFIERS"),
            Self::PAREN_EXPR => ::core::fmt::Formatter::write_str(f, "PAREN_EXPR"),
            Self::PATTERN => ::core::fmt::Formatter::write_str(f, "PATTERN"),
            Self::RECORD_FIELD => ::core::fmt::Formatter::write_str(f, "RECORD_FIELD"),
            Self::RECORD_NAME => ::core::fmt::Formatter::write_str(f, "RECORD_NAME"),
            Self::RECORD_TYPE => ::core::fmt::Formatter::write_str(f, "RECORD_TYPE"),
            Self::REL_COL => ::core::fmt::Formatter::write_str(f, "REL_COL"),
            Self::REL_COLS => ::core::fmt::Formatter::write_str(f, "REL_COLS"),
            Self::REL_KW => ::core::fmt::Formatter::write_str(f, "REL_KW"),
            Self::REL_NAME => ::core::fmt::Formatter::write_str(f, "REL_NAME"),
            Self::RELATION_DEF => ::core::fmt::Formatter::write_str(f, "RELATION_DEF"),
            Self::RET_EXPR => ::core::fmt::Formatter::write_str(f, "RET_EXPR"),
            Self::ROOT => ::core::fmt::Formatter::write_str(f, "ROOT"),
            Self::STMT => ::core::fmt::Formatter::write_str(f, "STMT"),
            Self::SUM_TYPE => ::core::fmt::Formatter::write_str(f, "SUM_TYPE"),
            Self::TUPLE_PATTERN => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN"),
            Self::TUPLE_PATTERN_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_PATTERN_ELEM"),
            Self::TUPLE_TYPE => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE"),
            Self::TUPLE_TYPE_ELEM => ::core::fmt::Formatter::write_str(f, "TUPLE_TYPE_ELEM"),
            Self::TYPE => ::core::fmt::Formatter::write_str(f, "TYPE"),
            Self::TYPE_BODY => ::core::fmt::Formatter::write_str(f, "TYPE_BODY"),
            Self::TYPE_DEF => ::core::fmt::Formatter::write_str(f, "TYPE_DEF"),
            Self::TYPE_NAME => ::core::fmt::Formatter::write_str(f, "TYPE_NAME"),
            Self::UNARY_EXPR => ::core::fmt::Formatter::write_str(f, "UNARY_EXPR"),
            Self::UNARY_OP => ::core::fmt::Formatter::write_str(f, "UNARY_OP"),
            Self::VAR_DECL => ::core::fmt::Formatter::write_str(f, "VAR_DECL"),
            Self::VAR_REF => ::core::fmt::Formatter::write_str(f, "VAR_REF"),
            Self::L_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '['),
            Self::R_BRACK => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, ']'),
            Self::CARET => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '^'),
            Self::AND => ::core::fmt::Formatter::write_str(f, "and"),
            Self::COMMENT => ::core::fmt::Formatter::write_str(f, "COMMENT"),
            Self::ELSE => ::core::fmt::Formatter::write_str(f, "else"),
            Self::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
            Self::EXTERN => ::core::fmt::Formatter::write_str(f, "extern"),
            Self::FALSE => ::core::fmt::Formatter::write_str(f, "false"),
            Self::FUNCTION => ::core::fmt::Formatter::write_str(f, "function"),
            Self::IDENT => ::core::fmt::Formatter::write_str(f, "IDENT"),
            Self::IF => ::core::fmt::Formatter::write_str(f, "if"),
            Self::INPUT => ::core::fmt::Formatter::write_str(f, "input"),
            Self::MULTISET => ::core::fmt::Formatter::write_str(f, "multiset"),
            Self::NUMBER => ::core::fmt::Formatter::write_str(f, "NUMBER"),
            Self::OR => ::core::fmt::Formatter::write_str(f, "or"),
            Self::OUTPUT => ::core::fmt::Formatter::write_str(f, "output"),
            Self::RELATION => ::core::fmt::Formatter::write_str(f, "relation"),
            Self::RETURN => ::core::fmt::Formatter::write_str(f, "return"),
            Self::STREAM => ::core::fmt::Formatter::write_str(f, "stream"),
            Self::STRING => ::core::fmt::Formatter::write_str(f, "STRING"),
            Self::TOMBSTONE => ::core::fmt::Formatter::write_str(f, "TOMBSTONE"),
            Self::TRUE => ::core::fmt::Formatter::write_str(f, "true"),
            Self::TYPEDEF => ::core::fmt::Formatter::write_str(f, "typedef"),
            Self::VAR => ::core::fmt::Formatter::write_str(f, "var"),
            Self::WHITESPACE => ::core::fmt::Formatter::write_str(f, "WHITESPACE"),
            Self::L_CURLY => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '{'),
            Self::PIPE => <::core::fmt::Formatter as ::core::fmt::Write>::write_char(f, '|'),
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
    (&) => {
        $crate::SyntaxKind::AMPERSAND
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
    (+) => {
        $crate::SyntaxKind::PLUS
    };
    (,) => {
        $crate::SyntaxKind::COMMA
    };
    (-) => {
        $crate::SyntaxKind::MINUS
    };
    (/) => {
        $crate::SyntaxKind::SLASH
    };
    (:) => {
        $crate::SyntaxKind::COLON
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
    ('[') => {
        $crate::SyntaxKind::L_BRACK
    };
    (']') => {
        $crate::SyntaxKind::R_BRACK
    };
    (^) => {
        $crate::SyntaxKind::CARET
    };
    (and) => {
        $crate::SyntaxKind::AND
    };
    (else) => {
        $crate::SyntaxKind::ELSE
    };
    (extern) => {
        $crate::SyntaxKind::EXTERN
    };
    (false) => {
        $crate::SyntaxKind::FALSE
    };
    (function) => {
        $crate::SyntaxKind::FUNCTION
    };
    (if) => {
        $crate::SyntaxKind::IF
    };
    (input) => {
        $crate::SyntaxKind::INPUT
    };
    (multiset) => {
        $crate::SyntaxKind::MULTISET
    };
    (or) => {
        $crate::SyntaxKind::OR
    };
    (output) => {
        $crate::SyntaxKind::OUTPUT
    };
    (relation) => {
        $crate::SyntaxKind::RELATION
    };
    (return) => {
        $crate::SyntaxKind::RETURN
    };
    (stream) => {
        $crate::SyntaxKind::STREAM
    };
    (true) => {
        $crate::SyntaxKind::TRUE
    };
    (typedef) => {
        $crate::SyntaxKind::TYPEDEF
    };
    (var) => {
        $crate::SyntaxKind::VAR
    };
    ('{') => {
        $crate::SyntaxKind::L_CURLY
    };
    (|) => {
        $crate::SyntaxKind::PIPE
    };
    ('}') => {
        $crate::SyntaxKind::R_CURLY
    };
}
