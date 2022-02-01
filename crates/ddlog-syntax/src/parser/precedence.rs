use crate::SyntaxKind;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[rustfmt::skip]
#[repr(u8)]
pub enum ExprPrecedence {
    As,
    Dot,
    Mul, Div, Mod, Pow,
    Add, Sub,
    Shl, Shr,
    Less, Greater, LessEq, GreaterEq,
    Eq, Ne,
    BitAnd,
    BitXor,
    BitOr,
    LogAnd,
    LogOr,
    Ternary,
    Assignment,
}

impl ExprPrecedence {
    #[rustfmt::skip]
    pub fn precedence(self) -> u8 {
        match self {
            Self::As              => 14,
            Self::Dot             => 13,
            Self::Mul
            | Self::Div
            | Self::Mod
            | Self::Pow           => 12,
            Self::Add | Self::Sub => 11,
            Self::Shl | Self::Shr => 10,
            Self::Less
            | Self::Greater
            | Self::LessEq
            | Self::GreaterEq     => 9,
            Self::Eq | Self::Ne   => 8,
            Self::BitAnd          => 7,
            Self::BitXor          => 6,
            Self::BitOr           => 5,
            Self::LogAnd          => 4,
            Self::LogOr           => 3,
            Self::Ternary         => 2,
            Self::Assignment      => 1,
        }
    }
}

impl TryFrom<SyntaxKind> for ExprPrecedence {
    type Error = ();

    fn try_from(kind: SyntaxKind) -> Result<ExprPrecedence, ()> {
        let precedence = match kind {
            T![as] => Self::As,
            T![.] => Self::Dot,
            T![*] => Self::Mul,
            T![/] => Self::Div,
            T![%] => Self::Mod,
            T![+] => Self::Add,
            T![-] => Self::Sub,
            T![<<] => Self::Shl,
            T![>>] => Self::Shr,
            T![<] => Self::Less,
            T![>] => Self::Greater,
            T![<=] => Self::LessEq,
            T![>=] => Self::GreaterEq,
            T![==] => Self::Eq,
            T![!=] => Self::Ne,
            T![&] => Self::BitAnd,
            T![^] => Self::BitXor,
            T![|] => Self::BitOr,
            T![and] => Self::LogAnd,
            T![or] => Self::LogOr,
            T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T![%=]
            | T![<<=]
            | T![>>=]
            | T![|=]
            | T![&=]
            | T![^=] => Self::Assignment,
            T![if] => Self::Ternary,
            _ => return Err(()),
        };

        Ok(precedence)
    }
}
