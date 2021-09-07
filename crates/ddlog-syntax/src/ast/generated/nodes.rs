#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Ampersand {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Ampersand {
    #[inline]
    pub fn ampersand(&self) -> ::core::option::Option<&crate::ast::tokens::Ampersand> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Ampersand {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AMPERSAND
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl And {
    #[inline]
    pub fn and(&self) -> ::core::option::Option<&crate::ast::tokens::And> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for And {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::AND
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Assign {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Assign {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pat>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<&crate::ast::tokens::Eq> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for Assign {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ASSIGN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Bang {
    #[inline]
    pub fn bang(&self) -> ::core::option::Option<&crate::ast::tokens::Bang> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Bang {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BANG
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct BinExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl BinExpr {
    #[inline]
    pub fn lhs(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn op(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::BinOp>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn rhs(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for BinExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BIN_EXPR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
    Ampersand(Ampersand),
    And(And),
    Caret(Caret),
    EqEq(EqEq),
    LAngle(LAngle),
    LAngleEq(LAngleEq),
    Minus(Minus),
    Neq(Neq),
    Or(Or),
    Percent(Percent),
    Pipe(Pipe),
    Plus(Plus),
    RAngle(RAngle),
    RAngleEq(RAngleEq),
    RightRocket(RightRocket),
    Shl(Shl),
    Shr(Shr),
    Slash(Slash),
    Star(Star),
}
impl BinOp {
    #[inline]
    pub fn as_plus(&self) -> ::core::option::Option<&Plus> {
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
    pub fn as_minus(&self) -> ::core::option::Option<&Minus> {
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
    pub fn as_star(&self) -> ::core::option::Option<&Star> {
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
    pub fn as_slash(&self) -> ::core::option::Option<&Slash> {
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
    pub fn as_percent(&self) -> ::core::option::Option<&Percent> {
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
    pub fn as_right_rocket(&self) -> ::core::option::Option<&RightRocket> {
        if let Self::RightRocket(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_right_rocket(&self) -> bool {
        ::core::matches!(self, Self::RightRocket(_))
    }
    #[inline]
    pub fn as_pipe(&self) -> ::core::option::Option<&Pipe> {
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
    pub fn as_caret(&self) -> ::core::option::Option<&Caret> {
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
    pub fn as_ampersand(&self) -> ::core::option::Option<&Ampersand> {
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
    pub fn as_shl(&self) -> ::core::option::Option<&Shl> {
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
    pub fn as_shr(&self) -> ::core::option::Option<&Shr> {
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
    pub fn as_and(&self) -> ::core::option::Option<&And> {
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
    pub fn as_or(&self) -> ::core::option::Option<&Or> {
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
    pub fn as_eq_eq(&self) -> ::core::option::Option<&EqEq> {
        if let Self::EqEq(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_eq_eq(&self) -> bool {
        ::core::matches!(self, Self::EqEq(_))
    }
    #[inline]
    pub fn as_neq(&self) -> ::core::option::Option<&Neq> {
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
    pub fn as_r_angle(&self) -> ::core::option::Option<&RAngle> {
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
    pub fn as_r_angle_eq(&self) -> ::core::option::Option<&RAngleEq> {
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
    pub fn as_l_angle(&self) -> ::core::option::Option<&LAngle> {
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
    pub fn as_l_angle_eq(&self) -> ::core::option::Option<&LAngleEq> {
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
impl crate::ast::AstNode for BinOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::AMPERSAND
                | crate::SyntaxKind::AND
                | crate::SyntaxKind::CARET
                | crate::SyntaxKind::EQ_EQ
                | crate::SyntaxKind::L_ANGLE
                | crate::SyntaxKind::L_ANGLE_EQ
                | crate::SyntaxKind::MINUS
                | crate::SyntaxKind::NEQ
                | crate::SyntaxKind::OR
                | crate::SyntaxKind::PERCENT
                | crate::SyntaxKind::PIPE
                | crate::SyntaxKind::PLUS
                | crate::SyntaxKind::R_ANGLE
                | crate::SyntaxKind::R_ANGLE_EQ
                | crate::SyntaxKind::RIGHT_ROCKET
                | crate::SyntaxKind::SHL
                | crate::SyntaxKind::SHR
                | crate::SyntaxKind::SLASH
                | crate::SyntaxKind::STAR
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::AMPERSAND => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Ampersand(
                    ::std::borrow::Cow::into_owned(Ampersand::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::AMPERSAND,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Ampersand),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::AND => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::And(
                    ::std::borrow::Cow::into_owned(And::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::AND,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(And),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::CARET => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Caret(
                    ::std::borrow::Cow::into_owned(Caret::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::CARET,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Caret),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::EQ_EQ => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::EqEq(
                    ::std::borrow::Cow::into_owned(EqEq::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::EQ_EQ,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(EqEq),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::L_ANGLE => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LAngle(
                    ::std::borrow::Cow::into_owned(LAngle::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::L_ANGLE,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(LAngle),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::L_ANGLE_EQ => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LAngleEq(
                    ::std::borrow::Cow::into_owned(LAngleEq::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::L_ANGLE_EQ,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(LAngleEq),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::MINUS => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Minus(
                    ::std::borrow::Cow::into_owned(Minus::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::MINUS,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Minus),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::NEQ => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Neq(
                    ::std::borrow::Cow::into_owned(Neq::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::NEQ,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Neq),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::OR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Or(
                    ::std::borrow::Cow::into_owned(Or::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::OR,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Or),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::PERCENT => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Percent(
                    ::std::borrow::Cow::into_owned(Percent::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::PERCENT,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Percent),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::PIPE => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Pipe(
                    ::std::borrow::Cow::into_owned(Pipe::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::PIPE,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Pipe),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::PLUS => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Plus(
                    ::std::borrow::Cow::into_owned(Plus::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::PLUS,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Plus),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::R_ANGLE => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RAngle(
                    ::std::borrow::Cow::into_owned(RAngle::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::R_ANGLE,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(RAngle),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::R_ANGLE_EQ => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RAngleEq(
                    ::std::borrow::Cow::into_owned(RAngleEq::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::R_ANGLE_EQ,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(RAngleEq),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::RIGHT_ROCKET => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::RightRocket(::std::borrow::Cow::into_owned(
                    RightRocket::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::RIGHT_ROCKET,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(RightRocket),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }),
                ))),
            ),
            crate::SyntaxKind::SHL => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Shl(
                    ::std::borrow::Cow::into_owned(Shl::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::SHL,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Shl),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::SHR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Shr(
                    ::std::borrow::Cow::into_owned(Shr::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::SHR,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Shr),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::SLASH => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Slash(
                    ::std::borrow::Cow::into_owned(Slash::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::SLASH,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Slash),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::STAR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Star(
                    ::std::borrow::Cow::into_owned(Star::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::STAR,
                                ::core::stringify!(BinOp),
                                ::core::stringify!(Star),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::Ampersand(node) => node.syntax(),
            Self::And(node) => node.syntax(),
            Self::Caret(node) => node.syntax(),
            Self::EqEq(node) => node.syntax(),
            Self::LAngle(node) => node.syntax(),
            Self::LAngleEq(node) => node.syntax(),
            Self::Minus(node) => node.syntax(),
            Self::Neq(node) => node.syntax(),
            Self::Or(node) => node.syntax(),
            Self::Percent(node) => node.syntax(),
            Self::Pipe(node) => node.syntax(),
            Self::Plus(node) => node.syntax(),
            Self::RAngle(node) => node.syntax(),
            Self::RAngleEq(node) => node.syntax(),
            Self::RightRocket(node) => node.syntax(),
            Self::Shl(node) => node.syntax(),
            Self::Shr(node) => node.syntax(),
            Self::Slash(node) => node.syntax(),
            Self::Star(node) => node.syntax(),
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
pub struct Block {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Block {
    #[inline]
    pub fn l_curly(&self) -> ::core::option::Option<&crate::ast::tokens::LCurly> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn stmt(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Stmt>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(&self) -> ::core::option::Option<&crate::ast::tokens::RCurly> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Block {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BLOCK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
    False(False),
    True(True),
}
impl Bool {
    #[inline]
    pub fn as_true(&self) -> ::core::option::Option<&True> {
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
    pub fn as_false(&self) -> ::core::option::Option<&False> {
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
impl crate::ast::AstNode for Bool {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(kind, crate::SyntaxKind::FALSE | crate::SyntaxKind::TRUE)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::FALSE => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::False(
                    ::std::borrow::Cow::into_owned(False::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::FALSE,
                                ::core::stringify!(Bool),
                                ::core::stringify!(False),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::TRUE => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::True(
                    ::std::borrow::Cow::into_owned(True::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::TRUE,
                                ::core::stringify!(Bool),
                                ::core::stringify!(True),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::False(node) => node.syntax(),
            Self::True(node) => node.syntax(),
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
pub struct Caret {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Caret {
    #[inline]
    pub fn caret(&self) -> ::core::option::Option<&crate::ast::tokens::Caret> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Caret {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CARET
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Colon {
    #[inline]
    pub fn colon(&self) -> ::core::option::Option<&crate::ast::tokens::Colon> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Colon {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COLON
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Comma {
    #[inline]
    pub fn comma(&self) -> ::core::option::Option<&crate::ast::tokens::Comma> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Comma {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMA
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ElseBlock {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ElseBlock {
    #[inline]
    pub fn else_token(&self) -> ::core::option::Option<&crate::ast::tokens::Else> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ElseBlock {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ELSE_BLOCK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Eq {
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<&crate::ast::tokens::Eq> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Eq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct EqEq {
    pub(crate) syntax: crate::SyntaxNode,
}
impl EqEq {
    #[inline]
    pub fn eqeq(&self) -> ::core::option::Option<&crate::ast::tokens::Eqeq> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for EqEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EQ_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
pub enum Expr {
    Assign(Assign),
    BinExpr(BinExpr),
    Block(Block),
    IfStmt(IfStmt),
    Literal(Literal),
    ParenExpr(ParenExpr),
    RetExpr(RetExpr),
    UnaryExpr(UnaryExpr),
    VarRef(VarRef),
}
impl Expr {
    #[inline]
    pub fn as_literal(&self) -> ::core::option::Option<&Literal> {
        if let Self::Literal(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_literal(&self) -> bool {
        ::core::matches!(self, Self::Literal(_))
    }
    #[inline]
    pub fn as_var_ref(&self) -> ::core::option::Option<&VarRef> {
        if let Self::VarRef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_var_ref(&self) -> bool {
        ::core::matches!(self, Self::VarRef(_))
    }
    #[inline]
    pub fn as_assign(&self) -> ::core::option::Option<&Assign> {
        if let Self::Assign(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_assign(&self) -> bool {
        ::core::matches!(self, Self::Assign(_))
    }
    #[inline]
    pub fn as_paren_expr(&self) -> ::core::option::Option<&ParenExpr> {
        if let Self::ParenExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_paren_expr(&self) -> bool {
        ::core::matches!(self, Self::ParenExpr(_))
    }
    #[inline]
    pub fn as_bin_expr(&self) -> ::core::option::Option<&BinExpr> {
        if let Self::BinExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_bin_expr(&self) -> bool {
        ::core::matches!(self, Self::BinExpr(_))
    }
    #[inline]
    pub fn as_if_stmt(&self) -> ::core::option::Option<&IfStmt> {
        if let Self::IfStmt(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_if_stmt(&self) -> bool {
        ::core::matches!(self, Self::IfStmt(_))
    }
    #[inline]
    pub fn as_ret_expr(&self) -> ::core::option::Option<&RetExpr> {
        if let Self::RetExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_ret_expr(&self) -> bool {
        ::core::matches!(self, Self::RetExpr(_))
    }
    #[inline]
    pub fn as_unary_expr(&self) -> ::core::option::Option<&UnaryExpr> {
        if let Self::UnaryExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_unary_expr(&self) -> bool {
        ::core::matches!(self, Self::UnaryExpr(_))
    }
    #[inline]
    pub fn as_block(&self) -> ::core::option::Option<&Block> {
        if let Self::Block(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_block(&self) -> bool {
        ::core::matches!(self, Self::Block(_))
    }
}
impl crate::ast::AstNode for Expr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::ASSIGN
                | crate::SyntaxKind::BIN_EXPR
                | crate::SyntaxKind::BLOCK
                | crate::SyntaxKind::IF_STMT
                | crate::SyntaxKind::LITERAL
                | crate::SyntaxKind::PAREN_EXPR
                | crate::SyntaxKind::RET_EXPR
                | crate::SyntaxKind::UNARY_EXPR
                | crate::SyntaxKind::VAR_REF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::ASSIGN => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Assign(
                    ::std::borrow::Cow::into_owned(Assign::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::ASSIGN,
                                ::core::stringify!(Expr),
                                ::core::stringify!(Assign),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::BIN_EXPR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::BinExpr(
                    ::std::borrow::Cow::into_owned(BinExpr::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::BIN_EXPR,
                                ::core::stringify!(Expr),
                                ::core::stringify!(BinExpr),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::BLOCK => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Block(
                    ::std::borrow::Cow::into_owned(Block::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::BLOCK,
                                ::core::stringify!(Expr),
                                ::core::stringify!(Block),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::IF_STMT => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::IfStmt(
                    ::std::borrow::Cow::into_owned(IfStmt::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::IF_STMT,
                                ::core::stringify!(Expr),
                                ::core::stringify!(IfStmt),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::LITERAL => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Literal(
                    ::std::borrow::Cow::into_owned(Literal::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::LITERAL,
                                ::core::stringify!(Expr),
                                ::core::stringify!(Literal),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::PAREN_EXPR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ParenExpr(
                    ::std::borrow::Cow::into_owned(ParenExpr::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::PAREN_EXPR,
                                ::core::stringify!(Expr),
                                ::core::stringify!(ParenExpr),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::RET_EXPR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RetExpr(
                    ::std::borrow::Cow::into_owned(RetExpr::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::RET_EXPR,
                                ::core::stringify!(Expr),
                                ::core::stringify!(RetExpr),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::UNARY_EXPR => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UnaryExpr(
                    ::std::borrow::Cow::into_owned(UnaryExpr::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::UNARY_EXPR,
                                ::core::stringify!(Expr),
                                ::core::stringify!(UnaryExpr),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::VAR_REF => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarRef(
                    ::std::borrow::Cow::into_owned(VarRef::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::VAR_REF,
                                ::core::stringify!(Expr),
                                ::core::stringify!(VarRef),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::Assign(node) => node.syntax(),
            Self::BinExpr(node) => node.syntax(),
            Self::Block(node) => node.syntax(),
            Self::IfStmt(node) => node.syntax(),
            Self::Literal(node) => node.syntax(),
            Self::ParenExpr(node) => node.syntax(),
            Self::RetExpr(node) => node.syntax(),
            Self::UnaryExpr(node) => node.syntax(),
            Self::VarRef(node) => node.syntax(),
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
pub struct ExprStmt {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ExprStmt {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(&self) -> ::core::option::Option<&crate::ast::tokens::Semicolon> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ExprStmt {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EXPR_STMT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Extern {
    #[inline]
    pub fn extern_token(&self) -> ::core::option::Option<&crate::ast::tokens::Extern> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Extern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EXTERN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl False {
    #[inline]
    pub fn false_token(&self) -> ::core::option::Option<&crate::ast::tokens::False> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for False {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FALSE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct FuncArg {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FuncArg {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pat>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(&self) -> ::core::option::Option<&crate::ast::tokens::Colon> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn comma(&self) -> ::core::option::Option<&crate::ast::tokens::Comma> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FuncArg {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNC_ARG
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct FuncArgs {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FuncArgs {
    #[inline]
    pub fn l_paren(&self) -> ::core::option::Option<&crate::ast::tokens::LParen> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn func_args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FuncArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(&self) -> ::core::option::Option<&crate::ast::tokens::RParen> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FuncArgs {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNC_ARGS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct FuncDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FuncDef {
    #[inline]
    pub fn modifiers(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FuncMods>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn keyword(&self) -> ::core::option::Option<&crate::ast::tokens::Function> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FuncName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn args(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FuncArgs>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(&self) -> ::core::option::Option<&crate::ast::tokens::Colon> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn return_ty(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FuncDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNC_DEF
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct FuncMods {
    pub(crate) syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for FuncMods {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNC_MODS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct FuncName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FuncName {
    #[inline]
    pub fn ident(&self) -> ::core::option::Option<&crate::ast::tokens::Ident> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FuncName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNC_NAME
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct IfBlock {
    pub(crate) syntax: crate::SyntaxNode,
}
impl IfBlock {
    #[inline]
    pub fn leading_else(&self) -> ::core::option::Option<&crate::ast::tokens::Else> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn if_token(&self) -> ::core::option::Option<&crate::ast::tokens::If> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn cond(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for IfBlock {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_BLOCK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct IfStmt {
    pub(crate) syntax: crate::SyntaxNode,
}
impl IfStmt {
    #[inline]
    pub fn if_blocks(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::IfBlock> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn else_block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ElseBlock>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for IfStmt {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_STMT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Input {
    #[inline]
    pub fn input(&self) -> ::core::option::Option<&crate::ast::tokens::Input> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Input {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::INPUT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
pub enum Item {
    FuncDef(FuncDef),
    RelationDef(RelationDef),
}
impl Item {
    #[inline]
    pub fn as_func_def(&self) -> ::core::option::Option<&FuncDef> {
        if let Self::FuncDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_func_def(&self) -> bool {
        ::core::matches!(self, Self::FuncDef(_))
    }
    #[inline]
    pub fn as_relation_def(&self) -> ::core::option::Option<&RelationDef> {
        if let Self::RelationDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_relation_def(&self) -> bool {
        ::core::matches!(self, Self::RelationDef(_))
    }
}
impl crate::ast::AstNode for Item {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::FUNC_DEF | crate::SyntaxKind::RELATION_DEF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::FUNC_DEF => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FuncDef(
                    ::std::borrow::Cow::into_owned(FuncDef::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::FUNC_DEF,
                                ::core::stringify!(Item),
                                ::core::stringify!(FuncDef),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::RELATION_DEF => ::core::option::Option::Some(
                ::std::borrow::Cow::Owned(Self::RelationDef(::std::borrow::Cow::into_owned(
                    RelationDef::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::RELATION_DEF,
                                ::core::stringify!(Item),
                                ::core::stringify!(RelationDef),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }),
                ))),
            ),
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::FuncDef(node) => node.syntax(),
            Self::RelationDef(node) => node.syntax(),
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
pub struct LAngle {
    pub(crate) syntax: crate::SyntaxNode,
}
impl LAngle {
    #[inline]
    pub fn l_angle(&self) -> ::core::option::Option<&crate::ast::tokens::LAngle> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for LAngle {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl LAngleEq {
    #[inline]
    pub fn l_angle_eq(&self) -> ::core::option::Option<&crate::ast::tokens::LAngleEq> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for LAngleEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_ANGLE_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl LBrack {
    #[inline]
    pub fn l_brack(&self) -> ::core::option::Option<&crate::ast::tokens::LBrack> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for LBrack {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_BRACK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl LCurly {
    #[inline]
    pub fn l_curly(&self) -> ::core::option::Option<&crate::ast::tokens::LCurly> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for LCurly {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_CURLY
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl LParen {
    #[inline]
    pub fn l_paren(&self) -> ::core::option::Option<&crate::ast::tokens::LParen> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for LParen {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::L_PAREN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
pub enum Literal {
    Bool(Bool),
    Number(Number),
    String(String),
}
impl Literal {
    #[inline]
    pub fn as_bool(&self) -> ::core::option::Option<&Bool> {
        if let Self::Bool(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_bool(&self) -> bool {
        ::core::matches!(self, Self::Bool(_))
    }
    #[inline]
    pub fn as_number(&self) -> ::core::option::Option<&Number> {
        if let Self::Number(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_number(&self) -> bool {
        ::core::matches!(self, Self::Number(_))
    }
    #[inline]
    pub fn as_string(&self) -> ::core::option::Option<&String> {
        if let Self::String(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_string(&self) -> bool {
        ::core::matches!(self, Self::String(_))
    }
}
impl crate::ast::AstNode for Literal {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::BOOL | crate::SyntaxKind::NUMBER | crate::SyntaxKind::STRING
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::BOOL => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Bool(
                    ::std::borrow::Cow::into_owned(Bool::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::BOOL,
                                ::core::stringify!(Literal),
                                ::core::stringify!(Bool),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::NUMBER => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Number(
                    ::std::borrow::Cow::into_owned(Number::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::NUMBER,
                                ::core::stringify!(Literal),
                                ::core::stringify!(Number),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::STRING => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::String(
                    ::std::borrow::Cow::into_owned(String::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::STRING,
                                ::core::stringify!(Literal),
                                ::core::stringify!(String),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::Bool(node) => node.syntax(),
            Self::Number(node) => node.syntax(),
            Self::String(node) => node.syntax(),
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
pub struct Minus {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Minus {
    #[inline]
    pub fn minus(&self) -> ::core::option::Option<&crate::ast::tokens::Minus> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Minus {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MINUS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Multiset {
    #[inline]
    pub fn multiset(&self) -> ::core::option::Option<&crate::ast::tokens::Multiset> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Multiset {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MULTISET
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Neq {
    #[inline]
    pub fn neq(&self) -> ::core::option::Option<&crate::ast::tokens::Neq> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Neq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NEQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Number {
    #[inline]
    pub fn number(&self) -> ::core::option::Option<&crate::ast::tokens::Number> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Number {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::NUMBER
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Or {
    #[inline]
    pub fn or(&self) -> ::core::option::Option<&crate::ast::tokens::Or> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Or {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Output {
    #[inline]
    pub fn output(&self) -> ::core::option::Option<&crate::ast::tokens::Output> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Output {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::OUTPUT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ParenExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ParenExpr {
    #[inline]
    pub fn l_paren(&self) -> ::core::option::Option<&crate::ast::tokens::LParen> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn inner(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn r_paren(&self) -> ::core::option::Option<&crate::ast::tokens::RParen> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ParenExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PAREN_EXPR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
pub enum Pat {
    TuplePat(TuplePat),
    VarRef(VarRef),
}
impl Pat {
    #[inline]
    pub fn as_var_ref(&self) -> ::core::option::Option<&VarRef> {
        if let Self::VarRef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_var_ref(&self) -> bool {
        ::core::matches!(self, Self::VarRef(_))
    }
    #[inline]
    pub fn as_tuple_pat(&self) -> ::core::option::Option<&TuplePat> {
        if let Self::TuplePat(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_tuple_pat(&self) -> bool {
        ::core::matches!(self, Self::TuplePat(_))
    }
}
impl crate::ast::AstNode for Pat {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::TUPLE_PAT | crate::SyntaxKind::VAR_REF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::TUPLE_PAT => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TuplePat(
                    ::std::borrow::Cow::into_owned(TuplePat::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::TUPLE_PAT,
                                ::core::stringify!(Pat),
                                ::core::stringify!(TuplePat),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::VAR_REF => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarRef(
                    ::std::borrow::Cow::into_owned(VarRef::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::VAR_REF,
                                ::core::stringify!(Pat),
                                ::core::stringify!(VarRef),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::TuplePat(node) => node.syntax(),
            Self::VarRef(node) => node.syntax(),
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
pub struct Percent {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Percent {
    #[inline]
    pub fn percent(&self) -> ::core::option::Option<&crate::ast::tokens::Percent> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Percent {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PERCENT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Pipe {
    #[inline]
    pub fn pipe(&self) -> ::core::option::Option<&crate::ast::tokens::Pipe> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Pipe {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PIPE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Plus {
    #[inline]
    pub fn plus(&self) -> ::core::option::Option<&crate::ast::tokens::Plus> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Plus {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PLUS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl RAngle {
    #[inline]
    pub fn r_angle(&self) -> ::core::option::Option<&crate::ast::tokens::RAngle> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RAngle {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl RAngleEq {
    #[inline]
    pub fn r_angle_eq(&self) -> ::core::option::Option<&crate::ast::tokens::RAngleEq> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RAngleEq {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_ANGLE_EQ
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl RBrack {
    #[inline]
    pub fn r_brack(&self) -> ::core::option::Option<&crate::ast::tokens::RBrack> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RBrack {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_BRACK
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl RCurly {
    #[inline]
    pub fn r_curly(&self) -> ::core::option::Option<&crate::ast::tokens::RCurly> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RCurly {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_CURLY
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl RParen {
    #[inline]
    pub fn r_paren(&self) -> ::core::option::Option<&crate::ast::tokens::RParen> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RParen {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::R_PAREN
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RelCol {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RelCol {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pat>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(&self) -> ::core::option::Option<&crate::ast::tokens::Colon> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn comma(&self) -> ::core::option::Option<&crate::ast::tokens::Comma> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RelCol {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::REL_COL
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RelCols {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RelCols {
    #[inline]
    pub fn l_paren(&self) -> ::core::option::Option<&crate::ast::tokens::LParen> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn rel_cols(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::RelCol> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(&self) -> ::core::option::Option<&crate::ast::tokens::RParen> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RelCols {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::REL_COLS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
    Multiset(Multiset),
    Relation(Relation),
    Stream(Stream),
}
impl RelKw {
    #[inline]
    pub fn as_relation(&self) -> ::core::option::Option<&Relation> {
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
    pub fn as_multiset(&self) -> ::core::option::Option<&Multiset> {
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
    pub fn as_stream(&self) -> ::core::option::Option<&Stream> {
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
impl crate::ast::AstNode for RelKw {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::MULTISET | crate::SyntaxKind::RELATION | crate::SyntaxKind::STREAM
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::MULTISET => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Multiset(
                    ::std::borrow::Cow::into_owned(Multiset::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::MULTISET,
                                ::core::stringify!(RelKw),
                                ::core::stringify!(Multiset),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::RELATION => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Relation(
                    ::std::borrow::Cow::into_owned(Relation::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::RELATION,
                                ::core::stringify!(RelKw),
                                ::core::stringify!(Relation),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::STREAM => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Stream(
                    ::std::borrow::Cow::into_owned(Stream::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::STREAM,
                                ::core::stringify!(RelKw),
                                ::core::stringify!(Stream),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::Multiset(node) => node.syntax(),
            Self::Relation(node) => node.syntax(),
            Self::Stream(node) => node.syntax(),
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
pub struct RelMods {
    pub(crate) syntax: crate::SyntaxNode,
}
impl crate::ast::AstNode for RelMods {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::REL_MODS
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RelName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RelName {
    #[inline]
    pub fn ident(&self) -> ::core::option::Option<&crate::ast::tokens::Ident> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RelName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::REL_NAME
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Relation {
    #[inline]
    pub fn relation(&self) -> ::core::option::Option<&crate::ast::tokens::Relation> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Relation {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RELATION
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RelationDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RelationDef {
    #[inline]
    pub fn modifiers(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RelMods>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RelKw>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RelName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn columns(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RelCols>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for RelationDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RELATION_DEF
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct RetExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RetExpr {
    #[inline]
    pub fn return_token(&self) -> ::core::option::Option<&crate::ast::tokens::Return> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for RetExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RET_EXPR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl RightRocket {
    #[inline]
    pub fn right_rocket(&self) -> ::core::option::Option<&crate::ast::tokens::RightRocket> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RightRocket {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RIGHT_ROCKET
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct Root {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Root {
    #[inline]
    pub fn item(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Item>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for Root {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ROOT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Semicolon {
    #[inline]
    pub fn semicolon(&self) -> ::core::option::Option<&crate::ast::tokens::Semicolon> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Semicolon {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SEMICOLON
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Shl {
    #[inline]
    pub fn shl(&self) -> ::core::option::Option<&crate::ast::tokens::Shl> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Shl {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHL
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Shr {
    #[inline]
    pub fn shr(&self) -> ::core::option::Option<&crate::ast::tokens::Shr> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Shr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SHR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Slash {
    #[inline]
    pub fn slash(&self) -> ::core::option::Option<&crate::ast::tokens::Slash> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Slash {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SLASH
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Star {
    #[inline]
    pub fn star(&self) -> ::core::option::Option<&crate::ast::tokens::Star> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Star {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STAR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
pub enum Stmt {
    ExprStmt(ExprStmt),
    IfStmt(IfStmt),
    VarDecl(VarDecl),
}
impl Stmt {
    #[inline]
    pub fn as_expr_stmt(&self) -> ::core::option::Option<&ExprStmt> {
        if let Self::ExprStmt(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_expr_stmt(&self) -> bool {
        ::core::matches!(self, Self::ExprStmt(_))
    }
    #[inline]
    pub fn as_var_decl(&self) -> ::core::option::Option<&VarDecl> {
        if let Self::VarDecl(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_var_decl(&self) -> bool {
        ::core::matches!(self, Self::VarDecl(_))
    }
    #[inline]
    pub fn as_if_stmt(&self) -> ::core::option::Option<&IfStmt> {
        if let Self::IfStmt(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_if_stmt(&self) -> bool {
        ::core::matches!(self, Self::IfStmt(_))
    }
}
impl crate::ast::AstNode for Stmt {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::EXPR_STMT | crate::SyntaxKind::IF_STMT | crate::SyntaxKind::VAR_DECL
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::EXPR_STMT => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ExprStmt(
                    ::std::borrow::Cow::into_owned(ExprStmt::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::EXPR_STMT,
                                ::core::stringify!(Stmt),
                                ::core::stringify!(ExprStmt),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::IF_STMT => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::IfStmt(
                    ::std::borrow::Cow::into_owned(IfStmt::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::IF_STMT,
                                ::core::stringify!(Stmt),
                                ::core::stringify!(IfStmt),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::VAR_DECL => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarDecl(
                    ::std::borrow::Cow::into_owned(VarDecl::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::VAR_DECL,
                                ::core::stringify!(Stmt),
                                ::core::stringify!(VarDecl),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::ExprStmt(node) => node.syntax(),
            Self::IfStmt(node) => node.syntax(),
            Self::VarDecl(node) => node.syntax(),
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
pub struct Stream {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Stream {
    #[inline]
    pub fn stream(&self) -> ::core::option::Option<&crate::ast::tokens::Stream> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Stream {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STREAM
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl String {
    #[inline]
    pub fn string(&self) -> ::core::option::Option<&crate::ast::tokens::String> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for String {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRING
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl True {
    #[inline]
    pub fn true_token(&self) -> ::core::option::Option<&crate::ast::tokens::True> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for True {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TRUE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct TuplePat {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TuplePat {
    #[inline]
    pub fn l_paren(&self) -> ::core::option::Option<&crate::ast::tokens::LParen> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn pat(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pat>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn comma(&self) -> ::core::option::Option<&crate::ast::tokens::Comma> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn r_paren(&self) -> ::core::option::Option<&crate::ast::tokens::RParen> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TuplePat {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_PAT
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
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
    pub(crate) syntax: crate::SyntaxNode,
}
impl Type {
    #[inline]
    pub fn ident(&self) -> ::core::option::Option<&crate::ast::tokens::Ident> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Type {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct UnaryExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl UnaryExpr {
    #[inline]
    pub fn op(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UnaryOp>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for UnaryExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::UNARY_EXPR
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
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
    Bang(Bang),
    Minus(Minus),
}
impl UnaryOp {
    #[inline]
    pub fn as_bang(&self) -> ::core::option::Option<&Bang> {
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
    pub fn as_minus(&self) -> ::core::option::Option<&Minus> {
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
impl crate::ast::AstNode for UnaryOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(kind, crate::SyntaxKind::BANG | crate::SyntaxKind::MINUS)
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::BANG => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Bang(
                    ::std::borrow::Cow::into_owned(Bang::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::BANG,
                                ::core::stringify!(UnaryOp),
                                ::core::stringify!(Bang),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            crate::SyntaxKind::MINUS => {
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Minus(
                    ::std::borrow::Cow::into_owned(Minus::cast(syntax).unwrap_or_else(|| {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting {} into a {}::{}",
                                crate::SyntaxKind::MINUS,
                                ::core::stringify!(UnaryOp),
                                ::core::stringify!(Minus),
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    })),
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::Bang(node) => node.syntax(),
            Self::Minus(node) => node.syntax(),
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
pub struct VarDecl {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VarDecl {
    #[inline]
    pub fn var(&self) -> ::core::option::Option<&crate::ast::tokens::Var> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pat>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<&crate::ast::tokens::Eq> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(&self) -> ::core::option::Option<&crate::ast::tokens::Semicolon> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VarDecl {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VAR_DECL
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct VarRef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VarRef {
    #[inline]
    pub fn ident(&self) -> ::core::option::Option<&crate::ast::tokens::Ident> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VarRef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VAR_REF
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        if <Self as crate::ast::AstNode>::can_cast_from(crate::SyntaxNode::kind(syntax)) {
            let node = unsafe { ::core::mem::transmute::<&crate::SyntaxNode, &Self>(syntax) };
            ::core::option::Option::Some(::std::borrow::Cow::Borrowed(node))
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        &self.syntax
    }
}
