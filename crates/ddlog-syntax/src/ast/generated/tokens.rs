#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(u8)]
pub enum AssignOp {
    AmpersandEq(crate::ast::tokens::AmpersandEq),
    CaretEq(crate::ast::tokens::CaretEq),
    Eq(crate::ast::tokens::Eq),
    MinusEq(crate::ast::tokens::MinusEq),
    PercentEq(crate::ast::tokens::PercentEq),
    PipeEq(crate::ast::tokens::PipeEq),
    PlusEq(crate::ast::tokens::PlusEq),
    ShlEq(crate::ast::tokens::ShlEq),
    ShrEq(crate::ast::tokens::ShrEq),
    SlashEq(crate::ast::tokens::SlashEq),
    StarEq(crate::ast::tokens::StarEq),
}
impl AssignOp {
    #[inline]
    pub fn as_eq(&self) -> ::core::option::Option<&crate::ast::tokens::Eq> {
        if let Self::Eq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_eq(&self) -> bool {
        ::core::matches!(self, Self::Eq(_))
    }
    #[inline]
    pub fn as_plus_eq(&self) -> ::core::option::Option<&crate::ast::tokens::PlusEq> {
        if let Self::PlusEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_plus_eq(&self) -> bool {
        ::core::matches!(self, Self::PlusEq(_))
    }
    #[inline]
    pub fn as_minus_eq(&self) -> ::core::option::Option<&crate::ast::tokens::MinusEq> {
        if let Self::MinusEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_minus_eq(&self) -> bool {
        ::core::matches!(self, Self::MinusEq(_))
    }
    #[inline]
    pub fn as_slash_eq(&self) -> ::core::option::Option<&crate::ast::tokens::SlashEq> {
        if let Self::SlashEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_slash_eq(&self) -> bool {
        ::core::matches!(self, Self::SlashEq(_))
    }
    #[inline]
    pub fn as_star_eq(&self) -> ::core::option::Option<&crate::ast::tokens::StarEq> {
        if let Self::StarEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_star_eq(&self) -> bool {
        ::core::matches!(self, Self::StarEq(_))
    }
    #[inline]
    pub fn as_percent_eq(&self) -> ::core::option::Option<&crate::ast::tokens::PercentEq> {
        if let Self::PercentEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_percent_eq(&self) -> bool {
        ::core::matches!(self, Self::PercentEq(_))
    }
    #[inline]
    pub fn as_ampersand_eq(&self) -> ::core::option::Option<&crate::ast::tokens::AmpersandEq> {
        if let Self::AmpersandEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_ampersand_eq(&self) -> bool {
        ::core::matches!(self, Self::AmpersandEq(_))
    }
    #[inline]
    pub fn as_pipe_eq(&self) -> ::core::option::Option<&crate::ast::tokens::PipeEq> {
        if let Self::PipeEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_pipe_eq(&self) -> bool {
        ::core::matches!(self, Self::PipeEq(_))
    }
    #[inline]
    pub fn as_caret_eq(&self) -> ::core::option::Option<&crate::ast::tokens::CaretEq> {
        if let Self::CaretEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_caret_eq(&self) -> bool {
        ::core::matches!(self, Self::CaretEq(_))
    }
    #[inline]
    pub fn as_shl_eq(&self) -> ::core::option::Option<&crate::ast::tokens::ShlEq> {
        if let Self::ShlEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_shl_eq(&self) -> bool {
        ::core::matches!(self, Self::ShlEq(_))
    }
    #[inline]
    pub fn as_shr_eq(&self) -> ::core::option::Option<&crate::ast::tokens::ShrEq> {
        if let Self::ShrEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_shr_eq(&self) -> bool {
        ::core::matches!(self, Self::ShrEq(_))
    }
}
impl crate::ast::AstToken for AssignOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::EQ
                | crate::SyntaxKind::PLUS_EQ
                | crate::SyntaxKind::MINUS_EQ
                | crate::SyntaxKind::SLASH_EQ
                | crate::SyntaxKind::STAR_EQ
                | crate::SyntaxKind::PERCENT_EQ
                | crate::SyntaxKind::AMPERSAND_EQ
                | crate::SyntaxKind::PIPE_EQ
                | crate::SyntaxKind::CARET_EQ
                | crate::SyntaxKind::SHL_EQ
                | crate::SyntaxKind::SHR_EQ
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            crate::SyntaxKind::EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Eq(crate::ast::tokens::Eq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::PLUS_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::PlusEq(crate::ast::tokens::PlusEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::MINUS_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::MinusEq(crate::ast::tokens::MinusEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::SLASH_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::SlashEq(crate::ast::tokens::SlashEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::STAR_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::StarEq(crate::ast::tokens::StarEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::PERCENT_EQ => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::PercentEq(crate::ast::tokens::PercentEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                })),
            ),
            crate::SyntaxKind::AMPERSAND_EQ => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::AmpersandEq(crate::ast::tokens::AmpersandEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                })),
            ),
            crate::SyntaxKind::PIPE_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::PipeEq(crate::ast::tokens::PipeEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::CARET_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::CaretEq(crate::ast::tokens::CaretEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::SHL_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::ShlEq(crate::ast::tokens::ShlEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::SHR_EQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::ShrEq(crate::ast::tokens::ShrEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Eq(syntax) => syntax.syntax(),
            Self::PlusEq(syntax) => syntax.syntax(),
            Self::MinusEq(syntax) => syntax.syntax(),
            Self::SlashEq(syntax) => syntax.syntax(),
            Self::StarEq(syntax) => syntax.syntax(),
            Self::PercentEq(syntax) => syntax.syntax(),
            Self::AmpersandEq(syntax) => syntax.syntax(),
            Self::PipeEq(syntax) => syntax.syntax(),
            Self::CaretEq(syntax) => syntax.syntax(),
            Self::ShlEq(syntax) => syntax.syntax(),
            Self::ShrEq(syntax) => syntax.syntax(),
        }
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(u8)]
pub enum BinOp {
    Ampersand(crate::ast::tokens::Ampersand),
    And(crate::ast::tokens::And),
    Caret(crate::ast::tokens::Caret),
    Eqeq(crate::ast::tokens::Eqeq),
    LAngle(crate::ast::tokens::LAngle),
    LAngleEq(crate::ast::tokens::LAngleEq),
    Minus(crate::ast::tokens::Minus),
    Neq(crate::ast::tokens::Neq),
    Or(crate::ast::tokens::Or),
    Percent(crate::ast::tokens::Percent),
    Pipe(crate::ast::tokens::Pipe),
    Plus(crate::ast::tokens::Plus),
    RAngle(crate::ast::tokens::RAngle),
    RAngleEq(crate::ast::tokens::RAngleEq),
    Shl(crate::ast::tokens::Shl),
    Shr(crate::ast::tokens::Shr),
    Slash(crate::ast::tokens::Slash),
    Star(crate::ast::tokens::Star),
}
impl BinOp {
    #[inline]
    pub fn as_plus(&self) -> ::core::option::Option<&crate::ast::tokens::Plus> {
        if let Self::Plus(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_plus(&self) -> bool {
        ::core::matches!(self, Self::Plus(_))
    }
    #[inline]
    pub fn as_minus(&self) -> ::core::option::Option<&crate::ast::tokens::Minus> {
        if let Self::Minus(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_minus(&self) -> bool {
        ::core::matches!(self, Self::Minus(_))
    }
    #[inline]
    pub fn as_star(&self) -> ::core::option::Option<&crate::ast::tokens::Star> {
        if let Self::Star(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_star(&self) -> bool {
        ::core::matches!(self, Self::Star(_))
    }
    #[inline]
    pub fn as_slash(&self) -> ::core::option::Option<&crate::ast::tokens::Slash> {
        if let Self::Slash(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_slash(&self) -> bool {
        ::core::matches!(self, Self::Slash(_))
    }
    #[inline]
    pub fn as_percent(&self) -> ::core::option::Option<&crate::ast::tokens::Percent> {
        if let Self::Percent(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_percent(&self) -> bool {
        ::core::matches!(self, Self::Percent(_))
    }
    #[inline]
    pub fn as_pipe(&self) -> ::core::option::Option<&crate::ast::tokens::Pipe> {
        if let Self::Pipe(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_pipe(&self) -> bool {
        ::core::matches!(self, Self::Pipe(_))
    }
    #[inline]
    pub fn as_caret(&self) -> ::core::option::Option<&crate::ast::tokens::Caret> {
        if let Self::Caret(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_caret(&self) -> bool {
        ::core::matches!(self, Self::Caret(_))
    }
    #[inline]
    pub fn as_ampersand(&self) -> ::core::option::Option<&crate::ast::tokens::Ampersand> {
        if let Self::Ampersand(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_ampersand(&self) -> bool {
        ::core::matches!(self, Self::Ampersand(_))
    }
    #[inline]
    pub fn as_shl(&self) -> ::core::option::Option<&crate::ast::tokens::Shl> {
        if let Self::Shl(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_shl(&self) -> bool {
        ::core::matches!(self, Self::Shl(_))
    }
    #[inline]
    pub fn as_shr(&self) -> ::core::option::Option<&crate::ast::tokens::Shr> {
        if let Self::Shr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_shr(&self) -> bool {
        ::core::matches!(self, Self::Shr(_))
    }
    #[inline]
    pub fn as_and(&self) -> ::core::option::Option<&crate::ast::tokens::And> {
        if let Self::And(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_and(&self) -> bool {
        ::core::matches!(self, Self::And(_))
    }
    #[inline]
    pub fn as_or(&self) -> ::core::option::Option<&crate::ast::tokens::Or> {
        if let Self::Or(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_or(&self) -> bool {
        ::core::matches!(self, Self::Or(_))
    }
    #[inline]
    pub fn as_eqeq(&self) -> ::core::option::Option<&crate::ast::tokens::Eqeq> {
        if let Self::Eqeq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_eqeq(&self) -> bool {
        ::core::matches!(self, Self::Eqeq(_))
    }
    #[inline]
    pub fn as_neq(&self) -> ::core::option::Option<&crate::ast::tokens::Neq> {
        if let Self::Neq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_neq(&self) -> bool {
        ::core::matches!(self, Self::Neq(_))
    }
    #[inline]
    pub fn as_r_angle(&self) -> ::core::option::Option<&crate::ast::tokens::RAngle> {
        if let Self::RAngle(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_r_angle(&self) -> bool {
        ::core::matches!(self, Self::RAngle(_))
    }
    #[inline]
    pub fn as_r_angle_eq(&self) -> ::core::option::Option<&crate::ast::tokens::RAngleEq> {
        if let Self::RAngleEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_r_angle_eq(&self) -> bool {
        ::core::matches!(self, Self::RAngleEq(_))
    }
    #[inline]
    pub fn as_l_angle(&self) -> ::core::option::Option<&crate::ast::tokens::LAngle> {
        if let Self::LAngle(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_l_angle(&self) -> bool {
        ::core::matches!(self, Self::LAngle(_))
    }
    #[inline]
    pub fn as_l_angle_eq(&self) -> ::core::option::Option<&crate::ast::tokens::LAngleEq> {
        if let Self::LAngleEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_l_angle_eq(&self) -> bool {
        ::core::matches!(self, Self::LAngleEq(_))
    }
}
impl crate::ast::AstToken for BinOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::PLUS
                | crate::SyntaxKind::MINUS
                | crate::SyntaxKind::STAR
                | crate::SyntaxKind::SLASH
                | crate::SyntaxKind::PERCENT
                | crate::SyntaxKind::PIPE
                | crate::SyntaxKind::CARET
                | crate::SyntaxKind::AMPERSAND
                | crate::SyntaxKind::SHL
                | crate::SyntaxKind::SHR
                | crate::SyntaxKind::AND
                | crate::SyntaxKind::OR
                | crate::SyntaxKind::EQEQ
                | crate::SyntaxKind::NEQ
                | crate::SyntaxKind::R_ANGLE
                | crate::SyntaxKind::R_ANGLE_EQ
                | crate::SyntaxKind::L_ANGLE
                | crate::SyntaxKind::L_ANGLE_EQ
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            crate::SyntaxKind::PLUS => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Plus(crate::ast::tokens::Plus {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::MINUS => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Minus(crate::ast::tokens::Minus {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::STAR => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Star(crate::ast::tokens::Star {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::SLASH => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Slash(crate::ast::tokens::Slash {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::PERCENT => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Percent(crate::ast::tokens::Percent {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::PIPE => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Pipe(crate::ast::tokens::Pipe {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::CARET => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Caret(crate::ast::tokens::Caret {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::AMPERSAND => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::Ampersand(crate::ast::tokens::Ampersand {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                })),
            ),
            crate::SyntaxKind::SHL => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Shl(crate::ast::tokens::Shl {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::SHR => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Shr(crate::ast::tokens::Shr {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::AND => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::And(crate::ast::tokens::And {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::OR => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Or(crate::ast::tokens::Or {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::EQEQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Eqeq(crate::ast::tokens::Eqeq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::NEQ => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Neq(crate::ast::tokens::Neq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::R_ANGLE => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::RAngle(crate::ast::tokens::RAngle {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::R_ANGLE_EQ => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::RAngleEq(crate::ast::tokens::RAngleEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                })),
            ),
            crate::SyntaxKind::L_ANGLE => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::LAngle(crate::ast::tokens::LAngle {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::L_ANGLE_EQ => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::LAngleEq(crate::ast::tokens::LAngleEq {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                })),
            ),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Plus(syntax) => syntax.syntax(),
            Self::Minus(syntax) => syntax.syntax(),
            Self::Star(syntax) => syntax.syntax(),
            Self::Slash(syntax) => syntax.syntax(),
            Self::Percent(syntax) => syntax.syntax(),
            Self::Pipe(syntax) => syntax.syntax(),
            Self::Caret(syntax) => syntax.syntax(),
            Self::Ampersand(syntax) => syntax.syntax(),
            Self::Shl(syntax) => syntax.syntax(),
            Self::Shr(syntax) => syntax.syntax(),
            Self::And(syntax) => syntax.syntax(),
            Self::Or(syntax) => syntax.syntax(),
            Self::Eqeq(syntax) => syntax.syntax(),
            Self::Neq(syntax) => syntax.syntax(),
            Self::RAngle(syntax) => syntax.syntax(),
            Self::RAngleEq(syntax) => syntax.syntax(),
            Self::LAngle(syntax) => syntax.syntax(),
            Self::LAngleEq(syntax) => syntax.syntax(),
        }
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(u8)]
pub enum Bool {
    False(crate::ast::tokens::False),
    True(crate::ast::tokens::True),
}
impl Bool {
    #[inline]
    pub fn as_true(&self) -> ::core::option::Option<&crate::ast::tokens::True> {
        if let Self::True(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_true(&self) -> bool {
        ::core::matches!(self, Self::True(_))
    }
    #[inline]
    pub fn as_false(&self) -> ::core::option::Option<&crate::ast::tokens::False> {
        if let Self::False(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_false(&self) -> bool {
        ::core::matches!(self, Self::False(_))
    }
}
impl crate::ast::AstToken for Bool {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(kind, crate::SyntaxKind::TRUE | crate::SyntaxKind::FALSE)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            crate::SyntaxKind::TRUE => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::True(crate::ast::tokens::True {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::FALSE => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::False(crate::ast::tokens::False {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::True(syntax) => syntax.syntax(),
            Self::False(syntax) => syntax.syntax(),
        }
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(u8)]
pub enum UnaryOp {
    Bang(crate::ast::tokens::Bang),
    Minus(crate::ast::tokens::Minus),
}
impl UnaryOp {
    #[inline]
    pub fn as_bang(&self) -> ::core::option::Option<&crate::ast::tokens::Bang> {
        if let Self::Bang(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_bang(&self) -> bool {
        ::core::matches!(self, Self::Bang(_))
    }
    #[inline]
    pub fn as_minus(&self) -> ::core::option::Option<&crate::ast::tokens::Minus> {
        if let Self::Minus(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_minus(&self) -> bool {
        ::core::matches!(self, Self::Minus(_))
    }
}
impl crate::ast::AstToken for UnaryOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(kind, crate::SyntaxKind::BANG | crate::SyntaxKind::MINUS)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            crate::SyntaxKind::BANG => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Bang(crate::ast::tokens::Bang {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::MINUS => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Minus(crate::ast::tokens::Minus {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Bang(syntax) => syntax.syntax(),
            Self::Minus(syntax) => syntax.syntax(),
        }
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Bang {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Bang {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BANG
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Neq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Neq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NEQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct HashBrack {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for HashBrack {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::HASH_BRACK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Percent {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Percent {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct PercentEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PercentEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Ampersand {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Ampersand {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct AmpersandEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AmpersandEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LParen {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LParen {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_PAREN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RParen {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RParen {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_PAREN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Star {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Star {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct StarEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StarEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Plus {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Plus {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct PlusEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PlusEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Comma {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Comma {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMA
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Minus {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Minus {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct MinusEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for MinusEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RightArrow {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RightArrow {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ARROW
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Dot {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Dot {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Slash {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Slash {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct SlashEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for SlashEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Colon {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Colon {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COLON
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct DoubleColon {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleColon {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_COLON
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Semicolon {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Semicolon {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SEMICOLON
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LAngle {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LAngle {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Shl {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Shl {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ShlEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ShlEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LAngleEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LAngleEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Eq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Eqeq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eqeq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQEQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RightRocket {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RightRocket {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ROCKET
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RAngle {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RAngle {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RAngleEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RAngleEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Shr {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Shr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ShrEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ShrEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LBrack {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LBrack {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_BRACK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RBrack {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RBrack {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_BRACK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Caret {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Caret {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct CaretEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for CaretEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct And {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for And {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AND
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct As {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for As {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Break {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Break {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BREAK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Const {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Const {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Continue {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Continue {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Else {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Else {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ELSE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Enum {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Enum {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct False {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for False {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FALSE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Fn {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Fn {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct For {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for For {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FOR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Ident {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Ident {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IDENT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct If {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for If {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Impl {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Impl {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct In {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for In {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Let {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Let {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LET
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Loop {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Loop {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LOOP
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Match {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Match {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Number {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Number {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NUMBER
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Or {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Or {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Pub {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Pub {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PUB
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Return {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Return {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RETURN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct String {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for String {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRING
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Struct {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Struct {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct True {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for True {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TRUE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Type {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Type {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Use {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Use {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct While {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for While {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHILE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct LCurly {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LCurly {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_CURLY
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Pipe {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Pipe {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct PipeEq {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PipeEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RCurly {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RCurly {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_CURLY
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
