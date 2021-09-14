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
    pub fn ampersand(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ampersand>> {
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
    pub fn and(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::And>> {
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
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
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
pub struct AttrPair {
    pub(crate) syntax: crate::SyntaxNode,
}
impl AttrPair {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for AttrPair {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTR_PAIR
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
pub struct Attribute {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Attribute {
    #[inline]
    pub fn hash_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::HashBrack>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn attr_pairs(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::AttrPair> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Attribute {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTRIBUTE
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
pub struct Attributes {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Attributes {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstNode for Attributes {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTRIBUTES
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
    pub fn bang(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Bang>> {
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
                let node = match Ampersand::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::AMPERSAND into a BinOp::Ampersand")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Ampersand(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::AND => {
                let node = match And::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::AND into a BinOp::And"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::And(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::CARET => {
                let node = match Caret::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::CARET into a BinOp::Caret")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Caret(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::EQ_EQ => {
                let node = match EqEq::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::EQ_EQ into a BinOp::EqEq")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::EqEq(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::L_ANGLE => {
                let node = match LAngle::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::L_ANGLE into a BinOp::LAngle")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LAngle(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::L_ANGLE_EQ => {
                let node = match LAngleEq::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::L_ANGLE_EQ into a BinOp::LAngleEq")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LAngleEq(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::MINUS => {
                let node = match Minus::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::MINUS into a BinOp::Minus")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Minus(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::NEQ => {
                let node = match Neq::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::NEQ into a BinOp::Neq"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Neq(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::OR => {
                let node = match Or::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::OR into a BinOp::Or"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Or(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::PERCENT => {
                let node = match Percent::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::PERCENT into a BinOp::Percent")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Percent(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::PIPE => {
                let node = match Pipe::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::PIPE into a BinOp::Pipe"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Pipe(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::PLUS => {
                let node = match Plus::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::PLUS into a BinOp::Plus"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Plus(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::R_ANGLE => {
                let node = match RAngle::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::R_ANGLE into a BinOp::RAngle")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RAngle(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::R_ANGLE_EQ => {
                let node = match RAngleEq::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::R_ANGLE_EQ into a BinOp::RAngleEq")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RAngleEq(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::RIGHT_ROCKET => {
                let node = match RightRocket::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::RIGHT_ROCKET into a BinOp::RightRocket")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RightRocket(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::SHL => {
                let node = match Shl::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::SHL into a BinOp::Shl"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Shl(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::SHR => {
                let node = match Shr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::SHR into a BinOp::Shr"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Shr(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::SLASH => {
                let node = match Slash::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::SLASH into a BinOp::Slash")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Slash(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::STAR => {
                let node = match Star::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::STAR into a BinOp::Star"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Star(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
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
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn statements(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Stmt> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
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
                let node = match False::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FALSE into a Bool::False")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::False(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::TRUE => {
                let node = match True::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            ::core::panic!(
                                "malformed codegen for casting SyntaxKind::TRUE into a Bool::True"
                            )
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::True(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
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
    pub fn caret(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Caret>> {
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
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
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
    pub fn comma(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Comma>> {
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
    pub fn else_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Else>> {
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
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
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
    pub fn eqeq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eqeq>> {
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
                let node = match Assign::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::ASSIGN into a Expr::Assign")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Assign(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::BIN_EXPR => {
                let node = match BinExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::BIN_EXPR into a Expr::BinExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::BinExpr(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::BLOCK => {
                let node = match Block::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::BLOCK into a Expr::Block")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Block(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::IF_STMT => {
                let node = match IfStmt::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::IF_STMT into a Expr::IfStmt")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::IfStmt(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::LITERAL => {
                let node = match Literal::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::LITERAL into a Expr::Literal")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Literal(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::PAREN_EXPR => {
                let node = match ParenExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::PAREN_EXPR into a Expr::ParenExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ParenExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::RET_EXPR => {
                let node = match RetExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::RET_EXPR into a Expr::RetExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RetExpr(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::UNARY_EXPR => {
                let node = match UnaryExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::UNARY_EXPR into a Expr::UnaryExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UnaryExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::VAR_REF => {
                let node = match VarRef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::VAR_REF into a Expr::VarRef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarRef(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
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
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
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
pub struct False {
    pub(crate) syntax: crate::SyntaxNode,
}
impl False {
    #[inline]
    pub fn false_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::False>> {
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
pub struct FunctionArg {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionArg {
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Attributes>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionArg {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_ARG
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
pub struct FunctionArgs {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionArgs {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FunctionArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionArgs {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_ARGS
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
pub struct FunctionDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionDef {
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Attributes>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn modifiers(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Modifiers>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Function>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn generics(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Generics>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn args(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionArgs>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn ret(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionReturn>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_DEF
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
pub struct FunctionName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_NAME
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
pub struct FunctionReturn {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionReturn {
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn return_ty(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionReturn {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_RETURN
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
pub struct FunctionReturnType {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionReturnType {
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn type_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionReturnType {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_RETURN_TYPE
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
pub struct FunctionType {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionType {
    #[inline]
    pub fn function(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Function>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionTypeArgs>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn ret(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FunctionReturnType>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionType {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_TYPE
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
pub struct FunctionTypeArg {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionTypeArg {
    #[inline]
    pub fn type_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionTypeArg {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_TYPE_ARG
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
pub struct FunctionTypeArgs {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionTypeArgs {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FunctionTypeArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionTypeArgs {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_TYPE_ARGS
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
pub struct GenericArg {
    pub(crate) syntax: crate::SyntaxNode,
}
impl GenericArg {
    #[inline]
    pub fn type_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for GenericArg {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::GENERIC_ARG
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
pub struct GenericType {
    pub(crate) syntax: crate::SyntaxNode,
}
impl GenericType {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn generics(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Generics>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for GenericType {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::GENERIC_TYPE
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
pub struct Generics {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Generics {
    #[inline]
    pub fn l_angle(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LAngle>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn generics(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::GenericArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_angle(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RAngle>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Generics {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::GENERICS
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
    pub fn leading_else(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Else>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn if_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::If>> {
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
#[repr(u8)]
pub enum Item {
    FunctionDef(FunctionDef),
    RelationDef(RelationDef),
    TypeDef(TypeDef),
}
impl Item {
    #[inline]
    pub fn as_function_def(&self) -> ::core::option::Option<&FunctionDef> {
        if let Self::FunctionDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_function_def(&self) -> bool {
        ::core::matches!(self, Self::FunctionDef(_))
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
    #[inline]
    pub fn as_type_def(&self) -> ::core::option::Option<&TypeDef> {
        if let Self::TypeDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_type_def(&self) -> bool {
        ::core::matches!(self, Self::TypeDef(_))
    }
}
impl crate::ast::AstNode for Item {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::FUNCTION_DEF
                | crate::SyntaxKind::RELATION_DEF
                | crate::SyntaxKind::TYPE_DEF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::FUNCTION_DEF => {
                let node = match FunctionDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FUNCTION_DEF into a Item::FunctionDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FunctionDef(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::RELATION_DEF => {
                let node = match RelationDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::RELATION_DEF into a Item::RelationDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RelationDef(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::TYPE_DEF => {
                let node = match TypeDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::TYPE_DEF into a Item::TypeDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TypeDef(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::FunctionDef(node) => node.syntax(),
            Self::RelationDef(node) => node.syntax(),
            Self::TypeDef(node) => node.syntax(),
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
    pub fn l_angle(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LAngle>> {
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
    pub fn l_angle_eq(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LAngleEq>> {
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
    pub fn l_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LBrack>> {
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
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
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
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
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
                let node = match Bool::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::BOOL into a Literal::Bool")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Bool(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::NUMBER => {
                let node = match Number::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::NUMBER into a Literal::Number")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Number(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::STRING => {
                let node = match String::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::STRING into a Literal::String")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::String(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
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
    pub fn minus(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Minus>> {
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
pub struct Modifiers {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Modifiers {
    #[inline]
    pub fn modifiers(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Modifier> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for Modifiers {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MODIFIERS
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
    pub fn neq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Neq>> {
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
    pub fn number(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Number>> {
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
    pub fn or(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Or>> {
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
pub struct ParenExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ParenExpr {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn inner(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
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
pub enum Pattern {
    TuplePattern(TuplePattern),
    VarRef(VarRef),
}
impl Pattern {
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
    pub fn as_tuple_pattern(&self) -> ::core::option::Option<&TuplePattern> {
        if let Self::TuplePattern(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_tuple_pattern(&self) -> bool {
        ::core::matches!(self, Self::TuplePattern(_))
    }
}
impl crate::ast::AstNode for Pattern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::TUPLE_PATTERN | crate::SyntaxKind::VAR_REF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::TUPLE_PATTERN => {
                let node = match TuplePattern::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::TUPLE_PATTERN into a Pattern::TuplePattern")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TuplePattern(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::VAR_REF => {
                let node = match VarRef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::VAR_REF into a Pattern::VarRef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarRef(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::TuplePattern(node) => node.syntax(),
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
    pub fn percent(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Percent>> {
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
    pub fn pipe(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pipe>> {
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
    pub fn plus(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Plus>> {
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
    pub fn r_angle(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RAngle>> {
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
    pub fn r_angle_eq(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RAngleEq>> {
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
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
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
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
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
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
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
pub struct RecordField {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RecordField {
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Attributes>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for RecordField {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RECORD_FIELD
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
pub struct RecordName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RecordName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RecordName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RECORD_NAME
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
pub struct RecordType {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RecordType {
    #[inline]
    pub fn constructor(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RecordName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::RecordField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for RecordType {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::RECORD_TYPE
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
    pub fn attributes(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Attributes>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
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
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn columns(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::RelCol> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
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
#[repr(transparent)]
pub struct RelName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RelName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
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
pub struct RelationDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl RelationDef {
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Attributes>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn modifiers(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Modifiers>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RelKw>> {
        crate::ast::support::token(&self.syntax)
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
    pub fn return_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Return>> {
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
    pub fn right_rocket(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightRocket>> {
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
    pub fn items(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Item> {
        crate::ast::support::children(&self.syntax)
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
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
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
    pub fn shl(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Shl>> {
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
    pub fn shr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Shr>> {
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
    pub fn slash(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Slash>> {
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
    pub fn star(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Star>> {
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
                let node = match ExprStmt::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::EXPR_STMT into a Stmt::ExprStmt")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ExprStmt(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::IF_STMT => {
                let node = match IfStmt::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::IF_STMT into a Stmt::IfStmt")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::IfStmt(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::VAR_DECL => {
                let node = match VarDecl::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::VAR_DECL into a Stmt::VarDecl")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VarDecl(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
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
pub struct String {
    pub(crate) syntax: crate::SyntaxNode,
}
impl String {
    #[inline]
    pub fn string(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::String>> {
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
pub struct SumType {
    pub(crate) syntax: crate::SyntaxNode,
}
impl SumType {
    #[inline]
    pub fn pipe(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pipe>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn record_type(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::RecordType>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for SumType {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::SUM_TYPE
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
    pub fn true_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::True>> {
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
pub struct TuplePattern {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TuplePattern {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn elements(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TuplePatternElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TuplePattern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_PATTERN
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
pub struct TuplePatternElem {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TuplePatternElem {
    #[inline]
    pub fn pattern(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TuplePatternElem {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_PATTERN_ELEM
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
pub struct TupleType {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TupleType {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn elements(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TupleTypeElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleType {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_TYPE
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
pub struct TupleTypeElem {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TupleTypeElem {
    #[inline]
    pub fn type_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleTypeElem {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_TYPE_ELEM
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
pub enum Type {
    FunctionType(FunctionType),
    GenericType(GenericType),
    TupleType(TupleType),
}
impl Type {
    #[inline]
    pub fn as_generic_type(&self) -> ::core::option::Option<&GenericType> {
        if let Self::GenericType(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_generic_type(&self) -> bool {
        ::core::matches!(self, Self::GenericType(_))
    }
    #[inline]
    pub fn as_tuple_type(&self) -> ::core::option::Option<&TupleType> {
        if let Self::TupleType(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_tuple_type(&self) -> bool {
        ::core::matches!(self, Self::TupleType(_))
    }
    #[inline]
    pub fn as_function_type(&self) -> ::core::option::Option<&FunctionType> {
        if let Self::FunctionType(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_function_type(&self) -> bool {
        ::core::matches!(self, Self::FunctionType(_))
    }
}
impl crate::ast::AstNode for Type {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::FUNCTION_TYPE
                | crate::SyntaxKind::GENERIC_TYPE
                | crate::SyntaxKind::TUPLE_TYPE
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::FUNCTION_TYPE => {
                let node = match FunctionType::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FUNCTION_TYPE into a Type::FunctionType")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FunctionType(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::GENERIC_TYPE => {
                let node = match GenericType::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::GENERIC_TYPE into a Type::GenericType")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::GenericType(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::TUPLE_TYPE => {
                let node = match TupleType::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::TUPLE_TYPE into a Type::TupleType")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TupleType(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::FunctionType(node) => node.syntax(),
            Self::GenericType(node) => node.syntax(),
            Self::TupleType(node) => node.syntax(),
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
pub enum TypeBody {
    RecordType(RecordType),
    SumType(SumType),
}
impl TypeBody {
    #[inline]
    pub fn as_record_type(&self) -> ::core::option::Option<&RecordType> {
        if let Self::RecordType(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_record_type(&self) -> bool {
        ::core::matches!(self, Self::RecordType(_))
    }
    #[inline]
    pub fn as_sum_type(&self) -> ::core::option::Option<&SumType> {
        if let Self::SumType(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_sum_type(&self) -> bool {
        ::core::matches!(self, Self::SumType(_))
    }
}
impl crate::ast::AstNode for TypeBody {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::RECORD_TYPE | crate::SyntaxKind::SUM_TYPE
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::RECORD_TYPE => {
                let node = match RecordType::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::RECORD_TYPE into a TypeBody::RecordType")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::RecordType(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::SUM_TYPE => {
                let node = match SumType::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::SUM_TYPE into a TypeBody::SumType")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::SumType(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::RecordType(node) => node.syntax(),
            Self::SumType(node) => node.syntax(),
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
pub struct TypeDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TypeDef {
    #[inline]
    pub fn attributes(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Attributes>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn modifiers(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Modifiers>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Typedef>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::TypeName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn body(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::TypeBody>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for TypeDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_DEF
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
pub struct TypeName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TypeName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TypeName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_NAME
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
                let node = match Bang::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::BANG into a UnaryOp::Bang")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Bang(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::MINUS => {
                let node = match Minus::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::MINUS into a UnaryOp::Minus")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::Minus(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
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
    pub fn var(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Var>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
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
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
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
