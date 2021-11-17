#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Ampersand {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Ampersand {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AMPERSAND`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AMPERSAND`]: crate::SyntaxKind::AMPERSAND
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AMPERSAND`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AMPERSAND`]: crate::SyntaxKind::AMPERSAND
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AmpersandEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AMPERSAND_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AMPERSAND_EQ`]: crate::SyntaxKind::AMPERSAND_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AMPERSAND_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AMPERSAND_EQ`]: crate::SyntaxKind::AMPERSAND_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for And {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AND`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AND`]: crate::SyntaxKind::AND
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AND
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AND`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AND`]: crate::SyntaxKind::AND
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct AsKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AsKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AS_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AS_KW`]: crate::SyntaxKind::AS_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AS_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AS_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AS_KW`]: crate::SyntaxKind::AS_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct Bang {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Bang {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BANG`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BANG`]: crate::SyntaxKind::BANG
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BANG
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`BANG`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`BANG`]: crate::SyntaxKind::BANG
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct BreakKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for BreakKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BREAK_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BREAK_KW`]: crate::SyntaxKind::BREAK_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BREAK_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`BREAK_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`BREAK_KW`]: crate::SyntaxKind::BREAK_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Caret {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CARET`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CARET`]: crate::SyntaxKind::CARET
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CARET`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CARET`]: crate::SyntaxKind::CARET
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for CaretEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CARET_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CARET_EQ`]: crate::SyntaxKind::CARET_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CARET_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CARET_EQ`]: crate::SyntaxKind::CARET_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Colon {
    /// Returns `true` if the given [`SyntaxKind`] is a [`COLON`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`COLON`]: crate::SyntaxKind::COLON
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COLON
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`COLON`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`COLON`]: crate::SyntaxKind::COLON
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Comma {
    /// Returns `true` if the given [`SyntaxKind`] is a [`COMMA`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`COMMA`]: crate::SyntaxKind::COMMA
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMA
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`COMMA`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`COMMA`]: crate::SyntaxKind::COMMA
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct Comment {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Comment {
    /// Returns `true` if the given [`SyntaxKind`] is a [`COMMENT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`COMMENT`]: crate::SyntaxKind::COMMENT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMENT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`COMMENT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`COMMENT`]: crate::SyntaxKind::COMMENT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct ConstKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ConstKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONST_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONST_KW`]: crate::SyntaxKind::CONST_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CONST_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CONST_KW`]: crate::SyntaxKind::CONST_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct ContinueKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ContinueKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONTINUE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONTINUE_KW`]: crate::SyntaxKind::CONTINUE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CONTINUE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CONTINUE_KW`]: crate::SyntaxKind::CONTINUE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Dot {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOT`]: crate::SyntaxKind::DOT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOT`]: crate::SyntaxKind::DOT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleColon {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOUBLE_COLON`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOUBLE_COLON`]: crate::SyntaxKind::DOUBLE_COLON
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_COLON
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOUBLE_COLON`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOUBLE_COLON`]: crate::SyntaxKind::DOUBLE_COLON
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct ElseKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ElseKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ELSE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ELSE_KW`]: crate::SyntaxKind::ELSE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ELSE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ELSE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ELSE_KW`]: crate::SyntaxKind::ELSE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct EnumKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for EnumKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ENUM_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ENUM_KW`]: crate::SyntaxKind::ENUM_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ENUM_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ENUM_KW`]: crate::SyntaxKind::ENUM_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct Eof {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eof {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EOF`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EOF`]: crate::SyntaxKind::EOF
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EOF
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`EOF`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`EOF`]: crate::SyntaxKind::EOF
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EQ`]: crate::SyntaxKind::EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`EQ`]: crate::SyntaxKind::EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eqeq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`EQEQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`EQEQ`]: crate::SyntaxKind::EQEQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQEQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`EQEQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`EQEQ`]: crate::SyntaxKind::EQEQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct Error {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Error {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ERROR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ERROR`]: crate::SyntaxKind::ERROR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ERROR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ERROR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ERROR`]: crate::SyntaxKind::ERROR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct FalseKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for FalseKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FALSE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FALSE_KW`]: crate::SyntaxKind::FALSE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FALSE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FALSE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FALSE_KW`]: crate::SyntaxKind::FALSE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct FnKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for FnKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FN_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FN_KW`]: crate::SyntaxKind::FN_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FN_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FN_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FN_KW`]: crate::SyntaxKind::FN_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct ForKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ForKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FOR_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FOR_KW`]: crate::SyntaxKind::FOR_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FOR_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FOR_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FOR_KW`]: crate::SyntaxKind::FOR_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for HashBrack {
    /// Returns `true` if the given [`SyntaxKind`] is a [`HASH_BRACK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`HASH_BRACK`]: crate::SyntaxKind::HASH_BRACK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::HASH_BRACK
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`HASH_BRACK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`HASH_BRACK`]: crate::SyntaxKind::HASH_BRACK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Ident {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IDENT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IDENT`]: crate::SyntaxKind::IDENT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IDENT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IDENT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IDENT`]: crate::SyntaxKind::IDENT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct IfKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for IfKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IF_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IF_KW`]: crate::SyntaxKind::IF_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IF_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IF_KW`]: crate::SyntaxKind::IF_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct ImplKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ImplKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IMPL_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IMPL_KW`]: crate::SyntaxKind::IMPL_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IMPL_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IMPL_KW`]: crate::SyntaxKind::IMPL_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct InKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for InKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IN_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IN_KW`]: crate::SyntaxKind::IN_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IN_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IN_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IN_KW`]: crate::SyntaxKind::IN_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LAngle {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_ANGLE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_ANGLE`]: crate::SyntaxKind::L_ANGLE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_ANGLE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_ANGLE`]: crate::SyntaxKind::L_ANGLE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LAngleEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_ANGLE_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_ANGLE_EQ`]: crate::SyntaxKind::L_ANGLE_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_ANGLE_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_ANGLE_EQ`]: crate::SyntaxKind::L_ANGLE_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LBrack {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_BRACK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_BRACK`]: crate::SyntaxKind::L_BRACK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_BRACK
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_BRACK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_BRACK`]: crate::SyntaxKind::L_BRACK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LCurly {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_CURLY`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_CURLY`]: crate::SyntaxKind::L_CURLY
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_CURLY
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_CURLY`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_CURLY`]: crate::SyntaxKind::L_CURLY
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LParen {
    /// Returns `true` if the given [`SyntaxKind`] is a [`L_PAREN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`L_PAREN`]: crate::SyntaxKind::L_PAREN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_PAREN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`L_PAREN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`L_PAREN`]: crate::SyntaxKind::L_PAREN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct LetKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LetKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LET_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LET_KW`]: crate::SyntaxKind::LET_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LET_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`LET_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`LET_KW`]: crate::SyntaxKind::LET_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct LoopKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LoopKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LOOP_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LOOP_KW`]: crate::SyntaxKind::LOOP_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LOOP_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`LOOP_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`LOOP_KW`]: crate::SyntaxKind::LOOP_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct MatchKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for MatchKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MATCH_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MATCH_KW`]: crate::SyntaxKind::MATCH_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MATCH_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MATCH_KW`]: crate::SyntaxKind::MATCH_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Minus {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MINUS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MINUS`]: crate::SyntaxKind::MINUS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MINUS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MINUS`]: crate::SyntaxKind::MINUS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for MinusEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MINUS_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MINUS_EQ`]: crate::SyntaxKind::MINUS_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MINUS_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MINUS_EQ`]: crate::SyntaxKind::MINUS_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Neq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`NEQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`NEQ`]: crate::SyntaxKind::NEQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NEQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`NEQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`NEQ`]: crate::SyntaxKind::NEQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct NumberLiteral {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for NumberLiteral {
    /// Returns `true` if the given [`SyntaxKind`] is a [`NUMBER_LITERAL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`NUMBER_LITERAL`]: crate::SyntaxKind::NUMBER_LITERAL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NUMBER_LITERAL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`NUMBER_LITERAL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`NUMBER_LITERAL`]: crate::SyntaxKind::NUMBER_LITERAL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Or {
    /// Returns `true` if the given [`SyntaxKind`] is a [`OR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`OR`]: crate::SyntaxKind::OR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`OR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`OR`]: crate::SyntaxKind::OR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Percent {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PERCENT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PERCENT`]: crate::SyntaxKind::PERCENT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PERCENT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PERCENT`]: crate::SyntaxKind::PERCENT
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PercentEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PERCENT_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PERCENT_EQ`]: crate::SyntaxKind::PERCENT_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PERCENT_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PERCENT_EQ`]: crate::SyntaxKind::PERCENT_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Pipe {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PIPE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PIPE`]: crate::SyntaxKind::PIPE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PIPE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PIPE`]: crate::SyntaxKind::PIPE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PipeEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PIPE_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PIPE_EQ`]: crate::SyntaxKind::PIPE_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PIPE_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PIPE_EQ`]: crate::SyntaxKind::PIPE_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Plus {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PLUS`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PLUS`]: crate::SyntaxKind::PLUS
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PLUS`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PLUS`]: crate::SyntaxKind::PLUS
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PlusEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PLUS_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PLUS_EQ`]: crate::SyntaxKind::PLUS_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PLUS_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PLUS_EQ`]: crate::SyntaxKind::PLUS_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct PubKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PubKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PUB_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PUB_KW`]: crate::SyntaxKind::PUB_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PUB_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PUB_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PUB_KW`]: crate::SyntaxKind::PUB_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RAngle {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_ANGLE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_ANGLE`]: crate::SyntaxKind::R_ANGLE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_ANGLE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_ANGLE`]: crate::SyntaxKind::R_ANGLE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RAngleEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_ANGLE_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_ANGLE_EQ`]: crate::SyntaxKind::R_ANGLE_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_ANGLE_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_ANGLE_EQ`]: crate::SyntaxKind::R_ANGLE_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RBrack {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_BRACK`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_BRACK`]: crate::SyntaxKind::R_BRACK
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_BRACK
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_BRACK`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_BRACK`]: crate::SyntaxKind::R_BRACK
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RCurly {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_CURLY`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_CURLY`]: crate::SyntaxKind::R_CURLY
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_CURLY
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_CURLY`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_CURLY`]: crate::SyntaxKind::R_CURLY
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RParen {
    /// Returns `true` if the given [`SyntaxKind`] is a [`R_PAREN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`R_PAREN`]: crate::SyntaxKind::R_PAREN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_PAREN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`R_PAREN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`R_PAREN`]: crate::SyntaxKind::R_PAREN
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct ReturnKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ReturnKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RETURN_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RETURN_KW`]: crate::SyntaxKind::RETURN_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RETURN_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RETURN_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RETURN_KW`]: crate::SyntaxKind::RETURN_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RightArrow {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RIGHT_ARROW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RIGHT_ARROW`]: crate::SyntaxKind::RIGHT_ARROW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ARROW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RIGHT_ARROW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RIGHT_ARROW`]: crate::SyntaxKind::RIGHT_ARROW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for RightRocket {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RIGHT_ROCKET`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RIGHT_ROCKET`]: crate::SyntaxKind::RIGHT_ROCKET
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ROCKET
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RIGHT_ROCKET`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RIGHT_ROCKET`]: crate::SyntaxKind::RIGHT_ROCKET
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Semicolon {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SEMICOLON`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SEMICOLON`]: crate::SyntaxKind::SEMICOLON
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SEMICOLON
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SEMICOLON`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SEMICOLON`]: crate::SyntaxKind::SEMICOLON
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Shl {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHL`]: crate::SyntaxKind::SHL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHL`]: crate::SyntaxKind::SHL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ShlEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHL_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHL_EQ`]: crate::SyntaxKind::SHL_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHL_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHL_EQ`]: crate::SyntaxKind::SHL_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Shr {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHR`]: crate::SyntaxKind::SHR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHR`]: crate::SyntaxKind::SHR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ShrEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SHR_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SHR_EQ`]: crate::SyntaxKind::SHR_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SHR_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SHR_EQ`]: crate::SyntaxKind::SHR_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Slash {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SLASH`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SLASH`]: crate::SyntaxKind::SLASH
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SLASH`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SLASH`]: crate::SyntaxKind::SLASH
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for SlashEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`SLASH_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`SLASH_EQ`]: crate::SyntaxKind::SLASH_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`SLASH_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`SLASH_EQ`]: crate::SyntaxKind::SLASH_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Star {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STAR`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STAR`]: crate::SyntaxKind::STAR
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STAR`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STAR`]: crate::SyntaxKind::STAR
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StarEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STAR_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STAR_EQ`]: crate::SyntaxKind::STAR_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STAR_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STAR_EQ`]: crate::SyntaxKind::STAR_EQ
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct StringLiteral {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StringLiteral {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRING_LITERAL`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRING_LITERAL`]: crate::SyntaxKind::STRING_LITERAL
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRING_LITERAL
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STRING_LITERAL`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STRING_LITERAL`]: crate::SyntaxKind::STRING_LITERAL
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct StructKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StructKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_KW`]: crate::SyntaxKind::STRUCT_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STRUCT_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STRUCT_KW`]: crate::SyntaxKind::STRUCT_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct Tombstone {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Tombstone {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TOMBSTONE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TOMBSTONE`]: crate::SyntaxKind::TOMBSTONE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TOMBSTONE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TOMBSTONE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TOMBSTONE`]: crate::SyntaxKind::TOMBSTONE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct TrueKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for TrueKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TRUE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TRUE_KW`]: crate::SyntaxKind::TRUE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TRUE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TRUE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TRUE_KW`]: crate::SyntaxKind::TRUE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct TypeKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for TypeKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TYPE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TYPE_KW`]: crate::SyntaxKind::TYPE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TYPE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TYPE_KW`]: crate::SyntaxKind::TYPE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct UseKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for UseKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`USE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`USE_KW`]: crate::SyntaxKind::USE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`USE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`USE_KW`]: crate::SyntaxKind::USE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct WhileKw {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for WhileKw {
    /// Returns `true` if the given [`SyntaxKind`] is a [`WHILE_KW`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`WHILE_KW`]: crate::SyntaxKind::WHILE_KW
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHILE_KW
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`WHILE_KW`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`WHILE_KW`]: crate::SyntaxKind::WHILE_KW
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
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
pub struct Whitespace {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Whitespace {
    /// Returns `true` if the given [`SyntaxKind`] is a [`WHITESPACE`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`WHITESPACE`]: crate::SyntaxKind::WHITESPACE
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHITESPACE
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`WHITESPACE`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`WHITESPACE`]: crate::SyntaxKind::WHITESPACE
    /// [`SyntaxKind`]: crate::SyntaxKind
    #[inline]
    fn cast(syntax: &crate::SyntaxToken) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstToken>::can_cast_from(crate::SyntaxToken::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxToken, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    ///Returns a reference to the inner [`SyntaxToken`][crate::SyntaxToken]
    #[inline]
    fn syntax(&self) -> &crate::SyntaxToken {
        &self.syntax
    }
}
