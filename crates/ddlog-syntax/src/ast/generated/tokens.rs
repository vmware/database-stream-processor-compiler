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
pub struct AndToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AndToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AND_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AND_TOKEN`]: crate::SyntaxKind::AND_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AND_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AND_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AND_TOKEN`]: crate::SyntaxKind::AND_TOKEN
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
pub struct AsToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AsToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`AS_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`AS_TOKEN`]: crate::SyntaxKind::AS_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AS_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`AS_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`AS_TOKEN`]: crate::SyntaxKind::AS_TOKEN
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
pub struct BreakToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for BreakToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`BREAK_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`BREAK_TOKEN`]: crate::SyntaxKind::BREAK_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BREAK_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`BREAK_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`BREAK_TOKEN`]: crate::SyntaxKind::BREAK_TOKEN
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
pub struct ConstToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ConstToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONST_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONST_TOKEN`]: crate::SyntaxKind::CONST_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CONST_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CONST_TOKEN`]: crate::SyntaxKind::CONST_TOKEN
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
pub struct ContinueToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ContinueToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`CONTINUE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`CONTINUE_TOKEN`]: crate::SyntaxKind::CONTINUE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`CONTINUE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`CONTINUE_TOKEN`]: crate::SyntaxKind::CONTINUE_TOKEN
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
pub struct DoubleDot {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleDot {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOUBLE_DOT`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOUBLE_DOT`]: crate::SyntaxKind::DOUBLE_DOT
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_DOT
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOUBLE_DOT`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOUBLE_DOT`]: crate::SyntaxKind::DOUBLE_DOT
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
pub struct DoubleDotEq {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for DoubleDotEq {
    /// Returns `true` if the given [`SyntaxKind`] is a [`DOUBLE_DOT_EQ`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`DOUBLE_DOT_EQ`]: crate::SyntaxKind::DOUBLE_DOT_EQ
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::DOUBLE_DOT_EQ
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`DOUBLE_DOT_EQ`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`DOUBLE_DOT_EQ`]: crate::SyntaxKind::DOUBLE_DOT_EQ
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
pub struct ElseToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ElseToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ELSE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ELSE_TOKEN`]: crate::SyntaxKind::ELSE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ELSE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ELSE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ELSE_TOKEN`]: crate::SyntaxKind::ELSE_TOKEN
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
pub struct EnumToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for EnumToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`ENUM_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`ENUM_TOKEN`]: crate::SyntaxKind::ENUM_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`ENUM_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`ENUM_TOKEN`]: crate::SyntaxKind::ENUM_TOKEN
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
pub struct FalseToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for FalseToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FALSE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FALSE_TOKEN`]: crate::SyntaxKind::FALSE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FALSE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FALSE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FALSE_TOKEN`]: crate::SyntaxKind::FALSE_TOKEN
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
pub struct FnToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for FnToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FN_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FN_TOKEN`]: crate::SyntaxKind::FN_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FN_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FN_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FN_TOKEN`]: crate::SyntaxKind::FN_TOKEN
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
pub struct ForToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ForToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`FOR_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`FOR_TOKEN`]: crate::SyntaxKind::FOR_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FOR_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`FOR_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`FOR_TOKEN`]: crate::SyntaxKind::FOR_TOKEN
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
pub struct IfToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for IfToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IF_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IF_TOKEN`]: crate::SyntaxKind::IF_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IF_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IF_TOKEN`]: crate::SyntaxKind::IF_TOKEN
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
pub struct ImplToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ImplToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IMPL_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IMPL_TOKEN`]: crate::SyntaxKind::IMPL_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IMPL_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IMPL_TOKEN`]: crate::SyntaxKind::IMPL_TOKEN
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
pub struct InToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for InToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`IN_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`IN_TOKEN`]: crate::SyntaxKind::IN_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IN_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`IN_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`IN_TOKEN`]: crate::SyntaxKind::IN_TOKEN
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
pub struct LetToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LetToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LET_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LET_TOKEN`]: crate::SyntaxKind::LET_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LET_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`LET_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`LET_TOKEN`]: crate::SyntaxKind::LET_TOKEN
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
pub struct LoopToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for LoopToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`LOOP_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`LOOP_TOKEN`]: crate::SyntaxKind::LOOP_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LOOP_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`LOOP_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`LOOP_TOKEN`]: crate::SyntaxKind::LOOP_TOKEN
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
pub struct MatchToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for MatchToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`MATCH_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`MATCH_TOKEN`]: crate::SyntaxKind::MATCH_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`MATCH_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`MATCH_TOKEN`]: crate::SyntaxKind::MATCH_TOKEN
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
pub struct OrToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for OrToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`OR_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`OR_TOKEN`]: crate::SyntaxKind::OR_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OR_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`OR_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`OR_TOKEN`]: crate::SyntaxKind::OR_TOKEN
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
pub struct PubToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PubToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`PUB_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`PUB_TOKEN`]: crate::SyntaxKind::PUB_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PUB_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`PUB_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`PUB_TOKEN`]: crate::SyntaxKind::PUB_TOKEN
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
pub struct ReturnToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ReturnToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`RETURN_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`RETURN_TOKEN`]: crate::SyntaxKind::RETURN_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RETURN_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`RETURN_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`RETURN_TOKEN`]: crate::SyntaxKind::RETURN_TOKEN
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
pub struct StructToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StructToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`STRUCT_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`STRUCT_TOKEN`]: crate::SyntaxKind::STRUCT_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`STRUCT_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`STRUCT_TOKEN`]: crate::SyntaxKind::STRUCT_TOKEN
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
pub struct TrueToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for TrueToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TRUE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TRUE_TOKEN`]: crate::SyntaxKind::TRUE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TRUE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TRUE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TRUE_TOKEN`]: crate::SyntaxKind::TRUE_TOKEN
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
pub struct TypeToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for TypeToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`TYPE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`TYPE_TOKEN`]: crate::SyntaxKind::TYPE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`TYPE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`TYPE_TOKEN`]: crate::SyntaxKind::TYPE_TOKEN
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
pub struct UseToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for UseToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`USE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`USE_TOKEN`]: crate::SyntaxKind::USE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`USE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`USE_TOKEN`]: crate::SyntaxKind::USE_TOKEN
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
pub struct WhileToken {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for WhileToken {
    /// Returns `true` if the given [`SyntaxKind`] is a [`WHILE_TOKEN`]
    /// [`SyntaxKind`]: crate::SyntaxKind
    /// [`WHILE_TOKEN`]: crate::SyntaxKind::WHILE_TOKEN
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHILE_TOKEN
    }
    /// Returns [`Some`] if the given [`SyntaxToken`] has the [`WHILE_TOKEN`] [`SyntaxKind`]
    /// [`Some`]: std::option::Option::Some
    /// [`SyntaxToken`]: crate::SyntaxToken
    /// [`WHILE_TOKEN`]: crate::SyntaxKind::WHILE_TOKEN
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
