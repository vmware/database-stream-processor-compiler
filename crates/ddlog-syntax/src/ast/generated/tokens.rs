#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(u8)]
pub enum Modifier {
    Extern(crate::ast::tokens::Extern),
    Input(crate::ast::tokens::Input),
    Output(crate::ast::tokens::Output),
}
impl Modifier {
    #[inline]
    pub fn as_input(&self) -> ::core::option::Option<&crate::ast::tokens::Input> {
        if let Self::Input(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_input(&self) -> bool {
        ::core::matches!(self, Self::Input(_))
    }
    #[inline]
    pub fn as_output(&self) -> ::core::option::Option<&crate::ast::tokens::Output> {
        if let Self::Output(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_output(&self) -> bool {
        ::core::matches!(self, Self::Output(_))
    }
    #[inline]
    pub fn as_extern(&self) -> ::core::option::Option<&crate::ast::tokens::Extern> {
        if let Self::Extern(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_extern(&self) -> bool {
        ::core::matches!(self, Self::Extern(_))
    }
}
impl crate::ast::AstToken for Modifier {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::INPUT | crate::SyntaxKind::OUTPUT | crate::SyntaxKind::EXTERN
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            crate::SyntaxKind::INPUT => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Input(crate::ast::tokens::Input {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::OUTPUT => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Output(crate::ast::tokens::Output {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::EXTERN => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Extern(crate::ast::tokens::Extern {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Input(syntax) => syntax.syntax(),
            Self::Output(syntax) => syntax.syntax(),
            Self::Extern(syntax) => syntax.syntax(),
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
pub enum RelKw {
    Multiset(crate::ast::tokens::Multiset),
    Relation(crate::ast::tokens::Relation),
    Stream(crate::ast::tokens::Stream),
}
impl RelKw {
    #[inline]
    pub fn as_relation(&self) -> ::core::option::Option<&crate::ast::tokens::Relation> {
        if let Self::Relation(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_relation(&self) -> bool {
        ::core::matches!(self, Self::Relation(_))
    }
    #[inline]
    pub fn as_multiset(&self) -> ::core::option::Option<&crate::ast::tokens::Multiset> {
        if let Self::Multiset(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_multiset(&self) -> bool {
        ::core::matches!(self, Self::Multiset(_))
    }
    #[inline]
    pub fn as_stream(&self) -> ::core::option::Option<&crate::ast::tokens::Stream> {
        if let Self::Stream(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_stream(&self) -> bool {
        ::core::matches!(self, Self::Stream(_))
    }
}
impl crate::ast::AstToken for RelKw {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::RELATION | crate::SyntaxKind::MULTISET | crate::SyntaxKind::STREAM
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxToken::kind(syntax) {
            crate::SyntaxKind::RELATION => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Relation(crate::ast::tokens::Relation {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::MULTISET => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Multiset(crate::ast::tokens::Multiset {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            crate::SyntaxKind::STREAM => ::core::option::Option::Some(::std::borrow::Cow::Owned(
                Self::Stream(crate::ast::tokens::Stream {
                    syntax: <crate::SyntaxToken as ::core::clone::Clone>::clone(syntax),
                }),
            )),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        match self {
            Self::Relation(syntax) => syntax.syntax(),
            Self::Multiset(syntax) => syntax.syntax(),
            Self::Stream(syntax) => syntax.syntax(),
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
pub struct Extern {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Extern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EXTERN
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
pub struct Function {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Function {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION
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
pub struct Input {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Input {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::INPUT
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
pub struct Multiset {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Multiset {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MULTISET
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
pub struct Output {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Output {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OUTPUT
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
pub struct Relation {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Relation {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RELATION
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
pub struct Stream {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Stream {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STREAM
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
pub struct Typedef {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Typedef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPEDEF
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
pub struct Var {
    pub(crate) syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Var {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VAR
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
