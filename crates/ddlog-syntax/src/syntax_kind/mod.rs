#[macro_use]
mod token_macro;
pub mod constants;
mod tests;

use logos::Logos;
use std::{
    fmt::{self, Display, Write},
    mem,
};

// TODO: This enum along with most of the surrounding utilities (`Debug`, `Display`, `T![]`, etc.)
//       should be auto-generated either by a proc macro or by an external script
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
#[allow(clippy::manual_non_exhaustive)]
#[repr(u16)]
pub enum SyntaxKind {
    /// The root of a syntax tree
    Root = 0,

    #[token(":")]
    Colon,

    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!=")]
    Neq,
    #[token("!")]
    Bang,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    // TODO: Since ddlog doesn't accept `&&` and `||` we might should
    //       still accept those as valid `And` and `Or` tokens and just
    //       filter for them in a validation pass where we inform the user
    //       that it's not accepted and they should use `and` and `or`
    //       (Possibly with a quickfix option to do so)
    #[token("and")]
    And,
    #[token("or")]
    Or,

    BinaryExpr,
    PrefixExpr,
    ParenExpr,
    Literal,
    NameRef,
    FnDecl,
    FnName,
    FnArgs,
    FnBody,

    /// An identifier
    // TODO: If we want more permissive identifiers we could switch to using
    //       `#[regex(r"[\p{XID_Start}][\p{XID_Continue}]*")]`
    //       which is standard unicode identifier syntax (remember to do NFKC
    //       normalization if this is ever done)
    #[regex("[A-Za-z_][A-Za-z0-9_]*")]
    Ident,
    /// A number, can be in either base10 (decimal), base2 (binary) or base16 (hex)
    #[regex("[0-9][0-9_]*")]
    #[regex("0b[0-1][0-1_]*")]
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*")]
    Number,
    /// A boolean
    #[token("true")]
    #[token("false")]
    Bool,

    /// The `var` keyword
    #[token("var")]
    VarKw,
    /// The `function` keyword
    #[token("function")]
    FnKw,
    /// The `relation` keyword
    #[token("relation")]
    RelKw,
    /// The `input` keyword
    #[token("input")]
    InputKw,
    /// The `output` keyword
    #[token("output")]
    OutputKw,
    /// The `typedef` keyword
    #[token("typedef")]
    TypedefKw,

    /// A comment
    // TODO: Block comments
    #[regex("//.*")]
    #[regex("///.*")]
    Comment,
    /// A piece of whitespace
    #[regex("[\n\t\r ]+")]
    Whitespace,
    /// An error within the syntax tree
    #[error]
    Error,
    /// The end of a file
    Eof,

    /// A tombstone event, used within parsing as a placeholder
    Tombstone,

    // Note: This must be the last variant of the enum so that it has
    //       the highest discriminant
    #[doc(hidden)]
    __LAST,
}

impl SyntaxKind {
    #[inline]
    pub const fn is_trivia(self) -> bool {
        matches!(self, T![whitespace] | T![comment])
    }
}

impl From<SyntaxKind> for cstree::SyntaxKind {
    #[inline]
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(kind: SyntaxKind) -> Self {
        kind as u16
    }
}

#[cold]
#[track_caller]
#[inline(never)]
fn invalid_syntax_kind(kind: u16) -> ! {
    panic!(
        "invalid SyntaxKind '{}', must be within the range of 0..={}",
        kind,
        SyntaxKind::__LAST as u16,
    )
}

impl From<u16> for SyntaxKind {
    #[inline]
    #[track_caller]
    fn from(kind: u16) -> Self {
        if kind > Self::__LAST as u16 {
            invalid_syntax_kind(kind)
        } else {
            // Safety: `kind` is a valid `SyntaxKind`
            unsafe { mem::transmute::<u16, Self>(kind) }
        }
    }
}

impl From<cstree::SyntaxKind> for SyntaxKind {
    #[inline]
    #[track_caller]
    fn from(cstree::SyntaxKind(kind): cstree::SyntaxKind) -> Self {
        if kind > Self::__LAST as u16 {
            invalid_syntax_kind(kind)
        } else {
            // Safety: `kind` is a valid `SyntaxKind`
            unsafe { mem::transmute::<u16, Self>(kind) }
        }
    }
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Colon => f.write_char(':'),
            Self::Eq => f.write_char('='),
            Self::EqEq => f.write_str("=="),
            Self::Neq => f.write_str("!="),
            Self::Bang => f.write_char('!'),
            Self::Plus => f.write_char('+'),
            Self::Minus => f.write_char('-'),
            Self::Star => f.write_char('*'),
            Self::Slash => f.write_char('/'),
            Self::LBrace => f.write_char('{'),
            Self::RBrace => f.write_char('}'),
            Self::LBracket => f.write_char('['),
            Self::RBracket => f.write_char(']'),
            Self::LParen => f.write_char('('),
            Self::RParen => f.write_char(')'),
            Self::And => f.write_str("and"),
            Self::Or => f.write_str("or"),
            Self::BinaryExpr => f.write_str("BINOP"),
            Self::PrefixExpr => f.write_str("PREFIX"),
            Self::ParenExpr => f.write_str("PARENS"),
            Self::Literal => f.write_str("LITERAL"),
            Self::NameRef => f.write_str("NAME_REF"),
            Self::FnName => f.write_str("FUNC_NAME"),
            Self::Ident => f.write_str("IDENT"),
            Self::Number => f.write_str("NUMBER"),
            Self::Bool => f.write_str("BOOL"),
            Self::FnDecl => f.write_str("FUNC_DECL"),
            Self::FnArgs => f.write_str("FUNC_ARGS"),
            Self::FnBody => f.write_str("FUNC_BODY"),
            Self::VarKw => f.write_str("var"),
            Self::FnKw => f.write_str("function"),
            Self::RelKw => f.write_str("relation"),
            Self::InputKw => f.write_str("input"),
            Self::OutputKw => f.write_str("output"),
            Self::TypedefKw => f.write_str("typedef"),
            Self::Comment => f.write_str("COMMENT"),
            Self::Whitespace => f.write_str("WS"),
            Self::Root => f.write_str("ROOT"),
            Self::Error => f.write_str("???"),
            Self::Eof => f.write_str("EOF"),
            Self::Tombstone => f.write_str("TOMBSTONE"),
            Self::__LAST => f.write_str("LAST"),
        }
    }
}
