#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct ArrayAccess {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ArrayAccess {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LBrack>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn index(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ArrayAccess {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ARRAY_ACCESS
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
pub struct ArrayExprElem {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ArrayExprElem {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for ArrayExprElem {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ARRAY_EXPR_ELEM
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
pub struct ArrayInitExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ArrayInitExpr {
    #[inline]
    pub fn l_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LBrack>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn array_expr_elem(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ArrayExprElem>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn r_brack(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RBrack>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ArrayInitExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ARRAY_INIT_EXPR
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
    pub fn assign_op(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::AssignOp>> {
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
pub struct AttrName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl AttrName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for AttrName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTR_NAME
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
    pub fn attr_name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::AttrName>> {
        crate::ast::support::child(&self.syntax)
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
pub struct BinExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl BinExpr {
    #[inline]
    pub fn lhs(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn op(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::BinOp>> {
        crate::ast::support::token(&self.syntax)
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
#[repr(transparent)]
pub struct BrackedStructField {
    pub(crate) syntax: crate::SyntaxNode,
}
impl BrackedStructField {
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::StructFieldName>> {
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
impl crate::ast::AstNode for BrackedStructField {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BRACKED_STRUCT_FIELD
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
pub struct BrackedStructFields {
    pub(crate) syntax: crate::SyntaxNode,
}
impl BrackedStructFields {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::BrackedStructField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for BrackedStructFields {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BRACKED_STRUCT_FIELDS
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
pub struct BreakExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl BreakExpr {
    #[inline]
    pub fn break_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Break>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for BreakExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BREAK_EXPR
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
pub struct ClosureArg {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ClosureArg {
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
impl crate::ast::AstNode for ClosureArg {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CLOSURE_ARG
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
pub struct ClosureExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ClosureExpr {
    #[inline]
    pub fn pipe(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pipe>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::ClosureArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn pipe(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pipe>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ClosureExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CLOSURE_EXPR
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
pub struct ConstDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ConstDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Const>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ConstName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn const_value(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ConstValue>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ConstDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_DEF
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
pub struct ConstName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ConstName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ConstName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_NAME
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
pub struct ConstValue {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ConstValue {
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ConstValue {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_VALUE
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
pub struct ContinueExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ContinueExpr {
    #[inline]
    pub fn continue_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Continue>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ContinueExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE_EXPR
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
pub struct EnumDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl EnumDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Enum>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::EnumName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn variants(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::EnumVariants>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_DEF
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
pub struct EnumName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl EnumName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_NAME
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
pub struct EnumVariant {
    pub(crate) syntax: crate::SyntaxNode,
}
impl EnumVariant {
    #[inline]
    pub fn enum_variant_name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::EnumVariantName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn enum_variant_body(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::EnumVariantBody>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumVariant {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANT
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
pub enum EnumVariantBody {
    VariantStruct(VariantStruct),
    VariantTuple(VariantTuple),
}
impl EnumVariantBody {
    #[inline]
    pub fn as_variant_tuple(&self) -> ::core::option::Option<&VariantTuple> {
        if let Self::VariantTuple(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_variant_tuple(&self) -> bool {
        ::core::matches!(self, Self::VariantTuple(_))
    }
    #[inline]
    pub fn as_variant_struct(&self) -> ::core::option::Option<&VariantStruct> {
        if let Self::VariantStruct(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_variant_struct(&self) -> bool {
        ::core::matches!(self, Self::VariantStruct(_))
    }
}
impl crate::ast::AstNode for EnumVariantBody {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::VARIANT_STRUCT | crate::SyntaxKind::VARIANT_TUPLE
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::VARIANT_STRUCT => {
                let node = match VariantStruct::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::VARIANT_STRUCT into a EnumVariantBody::VariantStruct")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VariantStruct(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::VARIANT_TUPLE => {
                let node = match VariantTuple::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::VARIANT_TUPLE into a EnumVariantBody::VariantTuple")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::VariantTuple(
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
            Self::VariantStruct(node) => node.syntax(),
            Self::VariantTuple(node) => node.syntax(),
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
pub struct EnumVariantName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl EnumVariantName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumVariantName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANT_NAME
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
pub struct EnumVariants {
    pub(crate) syntax: crate::SyntaxNode,
}
impl EnumVariants {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn variants(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::EnumVariant> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for EnumVariants {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANTS
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
    ArrayAccess(ArrayAccess),
    ArrayInitExpr(ArrayInitExpr),
    Assign(Assign),
    BinExpr(BinExpr),
    Block(Block),
    BreakExpr(BreakExpr),
    ClosureExpr(ClosureExpr),
    ContinueExpr(ContinueExpr),
    FieldAccess(FieldAccess),
    ForExpr(ForExpr),
    FunctionCall(FunctionCall),
    IfExpr(IfExpr),
    Literal(Literal),
    LoopExpr(LoopExpr),
    MatchExpr(MatchExpr),
    ParenExpr(ParenExpr),
    QualifiedRef(QualifiedRef),
    RetExpr(RetExpr),
    StructInitExpr(StructInitExpr),
    TupleInitExpr(TupleInitExpr),
    UnaryExpr(UnaryExpr),
    VarRef(VarRef),
    WhileExpr(WhileExpr),
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
    pub fn as_qualified_ref(&self) -> ::core::option::Option<&QualifiedRef> {
        if let Self::QualifiedRef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_qualified_ref(&self) -> bool {
        ::core::matches!(self, Self::QualifiedRef(_))
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
    pub fn as_if_expr(&self) -> ::core::option::Option<&IfExpr> {
        if let Self::IfExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_if_expr(&self) -> bool {
        ::core::matches!(self, Self::IfExpr(_))
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
    pub fn as_break_expr(&self) -> ::core::option::Option<&BreakExpr> {
        if let Self::BreakExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_break_expr(&self) -> bool {
        ::core::matches!(self, Self::BreakExpr(_))
    }
    #[inline]
    pub fn as_continue_expr(&self) -> ::core::option::Option<&ContinueExpr> {
        if let Self::ContinueExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_continue_expr(&self) -> bool {
        ::core::matches!(self, Self::ContinueExpr(_))
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
    #[inline]
    pub fn as_while_expr(&self) -> ::core::option::Option<&WhileExpr> {
        if let Self::WhileExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_while_expr(&self) -> bool {
        ::core::matches!(self, Self::WhileExpr(_))
    }
    #[inline]
    pub fn as_for_expr(&self) -> ::core::option::Option<&ForExpr> {
        if let Self::ForExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_for_expr(&self) -> bool {
        ::core::matches!(self, Self::ForExpr(_))
    }
    #[inline]
    pub fn as_loop_expr(&self) -> ::core::option::Option<&LoopExpr> {
        if let Self::LoopExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_loop_expr(&self) -> bool {
        ::core::matches!(self, Self::LoopExpr(_))
    }
    #[inline]
    pub fn as_match_expr(&self) -> ::core::option::Option<&MatchExpr> {
        if let Self::MatchExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_match_expr(&self) -> bool {
        ::core::matches!(self, Self::MatchExpr(_))
    }
    #[inline]
    pub fn as_closure_expr(&self) -> ::core::option::Option<&ClosureExpr> {
        if let Self::ClosureExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_closure_expr(&self) -> bool {
        ::core::matches!(self, Self::ClosureExpr(_))
    }
    #[inline]
    pub fn as_field_access(&self) -> ::core::option::Option<&FieldAccess> {
        if let Self::FieldAccess(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_field_access(&self) -> bool {
        ::core::matches!(self, Self::FieldAccess(_))
    }
    #[inline]
    pub fn as_array_access(&self) -> ::core::option::Option<&ArrayAccess> {
        if let Self::ArrayAccess(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_array_access(&self) -> bool {
        ::core::matches!(self, Self::ArrayAccess(_))
    }
    #[inline]
    pub fn as_function_call(&self) -> ::core::option::Option<&FunctionCall> {
        if let Self::FunctionCall(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_function_call(&self) -> bool {
        ::core::matches!(self, Self::FunctionCall(_))
    }
    #[inline]
    pub fn as_struct_init_expr(&self) -> ::core::option::Option<&StructInitExpr> {
        if let Self::StructInitExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_struct_init_expr(&self) -> bool {
        ::core::matches!(self, Self::StructInitExpr(_))
    }
    #[inline]
    pub fn as_tuple_init_expr(&self) -> ::core::option::Option<&TupleInitExpr> {
        if let Self::TupleInitExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_tuple_init_expr(&self) -> bool {
        ::core::matches!(self, Self::TupleInitExpr(_))
    }
    #[inline]
    pub fn as_array_init_expr(&self) -> ::core::option::Option<&ArrayInitExpr> {
        if let Self::ArrayInitExpr(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_array_init_expr(&self) -> bool {
        ::core::matches!(self, Self::ArrayInitExpr(_))
    }
}
impl crate::ast::AstNode for Expr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::ARRAY_ACCESS
                | crate::SyntaxKind::ARRAY_INIT_EXPR
                | crate::SyntaxKind::ASSIGN
                | crate::SyntaxKind::BIN_EXPR
                | crate::SyntaxKind::BLOCK
                | crate::SyntaxKind::BREAK_EXPR
                | crate::SyntaxKind::CLOSURE_EXPR
                | crate::SyntaxKind::CONTINUE_EXPR
                | crate::SyntaxKind::FIELD_ACCESS
                | crate::SyntaxKind::FOR_EXPR
                | crate::SyntaxKind::FUNCTION_CALL
                | crate::SyntaxKind::IF_EXPR
                | crate::SyntaxKind::LITERAL
                | crate::SyntaxKind::LOOP_EXPR
                | crate::SyntaxKind::MATCH_EXPR
                | crate::SyntaxKind::PAREN_EXPR
                | crate::SyntaxKind::QUALIFIED_REF
                | crate::SyntaxKind::RET_EXPR
                | crate::SyntaxKind::STRUCT_INIT_EXPR
                | crate::SyntaxKind::TUPLE_INIT_EXPR
                | crate::SyntaxKind::UNARY_EXPR
                | crate::SyntaxKind::VAR_REF
                | crate::SyntaxKind::WHILE_EXPR
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::ARRAY_ACCESS => {
                let node = match ArrayAccess::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::ARRAY_ACCESS into a Expr::ArrayAccess")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ArrayAccess(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::ARRAY_INIT_EXPR => {
                let node = match ArrayInitExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::ARRAY_INIT_EXPR into a Expr::ArrayInitExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ArrayInitExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
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
            crate::SyntaxKind::BREAK_EXPR => {
                let node = match BreakExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::BREAK_EXPR into a Expr::BreakExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::BreakExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::CLOSURE_EXPR => {
                let node = match ClosureExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::CLOSURE_EXPR into a Expr::ClosureExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ClosureExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::CONTINUE_EXPR => {
                let node = match ContinueExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::CONTINUE_EXPR into a Expr::ContinueExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ContinueExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::FIELD_ACCESS => {
                let node = match FieldAccess::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FIELD_ACCESS into a Expr::FieldAccess")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FieldAccess(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::FOR_EXPR => {
                let node = match ForExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FOR_EXPR into a Expr::ForExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ForExpr(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
            crate::SyntaxKind::FUNCTION_CALL => {
                let node = match FunctionCall::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FUNCTION_CALL into a Expr::FunctionCall")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FunctionCall(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::IF_EXPR => {
                let node = match IfExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::IF_EXPR into a Expr::IfExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::IfExpr(match node {
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
            crate::SyntaxKind::LOOP_EXPR => {
                let node = match LoopExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::LOOP_EXPR into a Expr::LoopExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::LoopExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::MATCH_EXPR => {
                let node = match MatchExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::MATCH_EXPR into a Expr::MatchExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::MatchExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
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
            crate::SyntaxKind::QUALIFIED_REF => {
                let node = match QualifiedRef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::QUALIFIED_REF into a Expr::QualifiedRef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::QualifiedRef(
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
            crate::SyntaxKind::STRUCT_INIT_EXPR => {
                let node = match StructInitExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::STRUCT_INIT_EXPR into a Expr::StructInitExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StructInitExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::TUPLE_INIT_EXPR => {
                let node = match TupleInitExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::TUPLE_INIT_EXPR into a Expr::TupleInitExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TupleInitExpr(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
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
            crate::SyntaxKind::WHILE_EXPR => {
                let node = match WhileExpr::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::WHILE_EXPR into a Expr::WhileExpr")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::WhileExpr(
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
            Self::ArrayAccess(node) => node.syntax(),
            Self::ArrayInitExpr(node) => node.syntax(),
            Self::Assign(node) => node.syntax(),
            Self::BinExpr(node) => node.syntax(),
            Self::Block(node) => node.syntax(),
            Self::BreakExpr(node) => node.syntax(),
            Self::ClosureExpr(node) => node.syntax(),
            Self::ContinueExpr(node) => node.syntax(),
            Self::FieldAccess(node) => node.syntax(),
            Self::ForExpr(node) => node.syntax(),
            Self::FunctionCall(node) => node.syntax(),
            Self::IfExpr(node) => node.syntax(),
            Self::Literal(node) => node.syntax(),
            Self::LoopExpr(node) => node.syntax(),
            Self::MatchExpr(node) => node.syntax(),
            Self::ParenExpr(node) => node.syntax(),
            Self::QualifiedRef(node) => node.syntax(),
            Self::RetExpr(node) => node.syntax(),
            Self::StructInitExpr(node) => node.syntax(),
            Self::TupleInitExpr(node) => node.syntax(),
            Self::UnaryExpr(node) => node.syntax(),
            Self::VarRef(node) => node.syntax(),
            Self::WhileExpr(node) => node.syntax(),
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
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
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
pub struct FieldAccess {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FieldAccess {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn dot(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Dot>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn field_accessor(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::FieldAccessor>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for FieldAccess {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FIELD_ACCESS
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
pub enum FieldAccessor {
    FieldAccessorName(FieldAccessorName),
    Number(Number),
}
impl FieldAccessor {
    #[inline]
    pub fn as_field_accessor_name(&self) -> ::core::option::Option<&FieldAccessorName> {
        if let Self::FieldAccessorName(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_field_accessor_name(&self) -> bool {
        ::core::matches!(self, Self::FieldAccessorName(_))
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
}
impl crate::ast::AstNode for FieldAccessor {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::FIELD_ACCESSOR_NAME | crate::SyntaxKind::NUMBER
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::FIELD_ACCESSOR_NAME => {
                let node = match FieldAccessorName::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::FIELD_ACCESSOR_NAME into a FieldAccessor::FieldAccessorName")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::FieldAccessorName(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::NUMBER => {
                let node = match Number::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::NUMBER into a FieldAccessor::Number")
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
            _ => ::core::option::Option::None,
        }
    }
    #[inline]
    fn syntax(&self) -> &crate::SyntaxNode {
        match self {
            Self::FieldAccessorName(node) => node.syntax(),
            Self::Number(node) => node.syntax(),
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
pub struct FieldAccessorName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FieldAccessorName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FieldAccessorName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FIELD_ACCESSOR_NAME
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
pub struct ForExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ForExpr {
    #[inline]
    pub fn for_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::For>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn in_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::In>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn iter(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ForExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FOR_EXPR
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
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
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
pub struct FunctionCall {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionCall {
    #[inline]
    pub fn func(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn args(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::FunctionCallArg> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionCall {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_CALL
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
pub struct FunctionCallArg {
    pub(crate) syntax: crate::SyntaxNode,
}
impl FunctionCallArg {
    #[inline]
    pub fn arg(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for FunctionCallArg {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_CALL_ARG
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
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Fn>> {
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
    pub fn right_arrow(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightArrow>> {
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
    pub fn right_arrow(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightArrow>> {
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
    pub fn fn_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Fn>> {
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
    pub fn path(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
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
pub struct IfExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl IfExpr {
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
impl crate::ast::AstNode for IfExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IF_EXPR
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
pub struct ImplBlock {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ImplBlock {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Impl>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn type_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn contents(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::ImplBlockContents>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for ImplBlock {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_BLOCK
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
pub struct ImplBlockContents {
    pub(crate) syntax: crate::SyntaxNode,
}
impl ImplBlockContents {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn items(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Item> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for ImplBlockContents {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::IMPL_BLOCK_CONTENTS
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
    ConstDef(ConstDef),
    EnumDef(EnumDef),
    FunctionDef(FunctionDef),
    ImplBlock(ImplBlock),
    StructDef(StructDef),
    TypeAlias(TypeAlias),
    UseDef(UseDef),
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
    pub fn as_struct_def(&self) -> ::core::option::Option<&StructDef> {
        if let Self::StructDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_struct_def(&self) -> bool {
        ::core::matches!(self, Self::StructDef(_))
    }
    #[inline]
    pub fn as_enum_def(&self) -> ::core::option::Option<&EnumDef> {
        if let Self::EnumDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_enum_def(&self) -> bool {
        ::core::matches!(self, Self::EnumDef(_))
    }
    #[inline]
    pub fn as_const_def(&self) -> ::core::option::Option<&ConstDef> {
        if let Self::ConstDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_const_def(&self) -> bool {
        ::core::matches!(self, Self::ConstDef(_))
    }
    #[inline]
    pub fn as_use_def(&self) -> ::core::option::Option<&UseDef> {
        if let Self::UseDef(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_use_def(&self) -> bool {
        ::core::matches!(self, Self::UseDef(_))
    }
    #[inline]
    pub fn as_impl_block(&self) -> ::core::option::Option<&ImplBlock> {
        if let Self::ImplBlock(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_impl_block(&self) -> bool {
        ::core::matches!(self, Self::ImplBlock(_))
    }
    #[inline]
    pub fn as_type_alias(&self) -> ::core::option::Option<&TypeAlias> {
        if let Self::TypeAlias(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_type_alias(&self) -> bool {
        ::core::matches!(self, Self::TypeAlias(_))
    }
}
impl crate::ast::AstNode for Item {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::CONST_DEF
                | crate::SyntaxKind::ENUM_DEF
                | crate::SyntaxKind::FUNCTION_DEF
                | crate::SyntaxKind::IMPL_BLOCK
                | crate::SyntaxKind::STRUCT_DEF
                | crate::SyntaxKind::TYPE_ALIAS
                | crate::SyntaxKind::USE_DEF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::CONST_DEF => {
                let node = match ConstDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::CONST_DEF into a Item::ConstDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ConstDef(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::ENUM_DEF => {
                let node = match EnumDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::ENUM_DEF into a Item::EnumDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::EnumDef(match node {
                    ::std::borrow::Cow::Owned(owned) => owned,
                    ::std::borrow::Cow::Borrowed(borrowed) => ::core::clone::Clone::clone(borrowed),
                })))
            }
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
            crate::SyntaxKind::IMPL_BLOCK => {
                let node = match ImplBlock::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::IMPL_BLOCK into a Item::ImplBlock")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::ImplBlock(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::STRUCT_DEF => {
                let node = match StructDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::STRUCT_DEF into a Item::StructDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StructDef(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::TYPE_ALIAS => {
                let node = match TypeAlias::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::TYPE_ALIAS into a Item::TypeAlias")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TypeAlias(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::USE_DEF => {
                let node = match UseDef::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::USE_DEF into a Item::UseDef")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UseDef(match node {
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
            Self::ConstDef(node) => node.syntax(),
            Self::EnumDef(node) => node.syntax(),
            Self::FunctionDef(node) => node.syntax(),
            Self::ImplBlock(node) => node.syntax(),
            Self::StructDef(node) => node.syntax(),
            Self::TypeAlias(node) => node.syntax(),
            Self::UseDef(node) => node.syntax(),
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
pub struct LoopExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl LoopExpr {
    #[inline]
    pub fn loop_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Loop>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn block(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Block>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for LoopExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LOOP_EXPR
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
pub struct MatchArm {
    pub(crate) syntax: crate::SyntaxNode,
}
impl MatchArm {
    #[inline]
    pub fn binding(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn right_rocket(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RightRocket>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn body(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for MatchArm {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_ARM
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
pub struct MatchExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl MatchExpr {
    #[inline]
    pub fn match_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Match>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn scrutinee(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn match_arms(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::MatchArm> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for MatchExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MATCH_EXPR
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
pub struct Modifier {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Modifier {
    #[inline]
    pub fn pub_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Pub>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for Modifier {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MODIFIER
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
#[repr(transparent)]
pub struct Path {
    pub(crate) syntax: crate::SyntaxNode,
}
impl Path {
    #[inline]
    pub fn double_colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::DoubleColon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn head(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::PathSegment>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn tail(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::PathTail> {
        crate::ast::support::children(&self.syntax)
    }
}
impl crate::ast::AstNode for Path {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATH
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
pub struct PathSegment {
    pub(crate) syntax: crate::SyntaxNode,
}
impl PathSegment {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for PathSegment {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATH_SEGMENT
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
pub struct PathTail {
    pub(crate) syntax: crate::SyntaxNode,
}
impl PathTail {
    #[inline]
    pub fn double_colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::DoubleColon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn tail(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::PathSegment>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for PathTail {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATH_TAIL
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
    StructPattern(StructPattern),
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
    #[inline]
    pub fn as_struct_pattern(&self) -> ::core::option::Option<&StructPattern> {
        if let Self::StructPattern(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_struct_pattern(&self) -> bool {
        ::core::matches!(self, Self::StructPattern(_))
    }
}
impl crate::ast::AstNode for Pattern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::STRUCT_PATTERN
                | crate::SyntaxKind::TUPLE_PATTERN
                | crate::SyntaxKind::VAR_REF
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::STRUCT_PATTERN => {
                let node = match StructPattern::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::STRUCT_PATTERN into a Pattern::StructPattern")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::StructPattern(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
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
            Self::StructPattern(node) => node.syntax(),
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
pub struct QualifiedRef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl QualifiedRef {
    #[inline]
    pub fn path(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for QualifiedRef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::QUALIFIED_REF
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
#[repr(u8)]
pub enum Stmt {
    ExprStmt(ExprStmt),
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
}
impl crate::ast::AstNode for Stmt {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::EXPR_STMT | crate::SyntaxKind::VAR_DECL
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
pub struct StructDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Struct>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::StructName>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::StructFields>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for StructDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_DEF
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
pub struct StructFieldName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructFieldName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructFieldName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_FIELD_NAME
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
pub enum StructFields {
    BrackedStructFields(BrackedStructFields),
    TupleStructFields(TupleStructFields),
}
impl StructFields {
    #[inline]
    pub fn as_bracked_struct_fields(&self) -> ::core::option::Option<&BrackedStructFields> {
        if let Self::BrackedStructFields(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_bracked_struct_fields(&self) -> bool {
        ::core::matches!(self, Self::BrackedStructFields(_))
    }
    #[inline]
    pub fn as_tuple_struct_fields(&self) -> ::core::option::Option<&TupleStructFields> {
        if let Self::TupleStructFields(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_tuple_struct_fields(&self) -> bool {
        ::core::matches!(self, Self::TupleStructFields(_))
    }
}
impl crate::ast::AstNode for StructFields {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::BRACKED_STRUCT_FIELDS | crate::SyntaxKind::TUPLE_STRUCT_FIELDS
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::BRACKED_STRUCT_FIELDS => {
                let node = match BrackedStructFields::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::BRACKED_STRUCT_FIELDS into a StructFields::BrackedStructFields")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::BrackedStructFields(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::TUPLE_STRUCT_FIELDS => {
                let node = match TupleStructFields::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::TUPLE_STRUCT_FIELDS into a StructFields::TupleStructFields")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::TupleStructFields(
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
            Self::BrackedStructFields(node) => node.syntax(),
            Self::TupleStructFields(node) => node.syntax(),
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
pub struct StructInitExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructInitExpr {
    #[inline]
    pub fn ty(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::StructInitField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructInitExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_INIT_EXPR
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
pub struct StructInitField {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructInitField {
    #[inline]
    pub fn field(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn value(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for StructInitField {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_INIT_FIELD
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
pub struct StructName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_NAME
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
pub struct StructPattern {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructPattern {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::StructPatternField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructPattern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_PATTERN
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
pub struct StructPatternField {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructPatternField {
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::StructPatternFieldName>>
    {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn colon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Colon>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn alias(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Pattern>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for StructPatternField {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_PATTERN_FIELD
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
pub struct StructPatternFieldName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl StructPatternFieldName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for StructPatternFieldName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_PATTERN_FIELD_NAME
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
pub struct TupleExprElem {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TupleExprElem {
    #[inline]
    pub fn expr(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Expr>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleExprElem {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_EXPR_ELEM
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
pub struct TupleInitExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TupleInitExpr {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn elems(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TupleExprElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleInitExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_INIT_EXPR
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
pub struct TupleStructField {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TupleStructField {
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
impl crate::ast::AstNode for TupleStructField {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_STRUCT_FIELD
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
pub struct TupleStructFields {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TupleStructFields {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn tuple_struct_fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::TupleStructField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for TupleStructFields {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TUPLE_STRUCT_FIELDS
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
#[repr(transparent)]
pub struct TypeAlias {
    pub(crate) syntax: crate::SyntaxNode,
}
impl TypeAlias {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Type>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn alias(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn eq(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Eq>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn original(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
    }
}
impl crate::ast::AstNode for TypeAlias {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TYPE_ALIAS
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
    pub fn op(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::UnaryOp>> {
        crate::ast::support::token(&self.syntax)
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
#[repr(transparent)]
pub struct UseAlias {
    pub(crate) syntax: crate::SyntaxNode,
}
impl UseAlias {
    #[inline]
    pub fn as_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::As>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn use_alias_name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UseAliasName>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for UseAlias {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_ALIAS
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
pub struct UseAliasName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl UseAliasName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for UseAliasName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_ALIAS_NAME
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
pub struct UseBranch {
    pub(crate) syntax: crate::SyntaxNode,
}
impl UseBranch {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn use_tree(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UseTree>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn commas(&self) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Comma> {
        crate::ast::support::token_children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for UseBranch {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_BRANCH
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
pub enum UseBranchOrAlias {
    UseAlias(UseAlias),
    UseBranch(UseBranch),
}
impl UseBranchOrAlias {
    #[inline]
    pub fn as_use_branch(&self) -> ::core::option::Option<&UseBranch> {
        if let Self::UseBranch(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_use_branch(&self) -> bool {
        ::core::matches!(self, Self::UseBranch(_))
    }
    #[inline]
    pub fn as_use_alias(&self) -> ::core::option::Option<&UseAlias> {
        if let Self::UseAlias(node) = self {
            ::core::option::Option::Some(node)
        } else {
            ::core::option::Option::None
        }
    }
    #[inline]
    pub fn is_use_alias(&self) -> bool {
        ::core::matches!(self, Self::UseAlias(_))
    }
}
impl crate::ast::AstNode for UseBranchOrAlias {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        ::core::matches!(
            kind,
            crate::SyntaxKind::USE_ALIAS | crate::SyntaxKind::USE_BRANCH
        )
    }
    #[inline]
    fn cast(syntax: &crate::SyntaxNode) -> ::core::option::Option<::std::borrow::Cow<'_, Self>> {
        match crate::SyntaxNode::kind(syntax) {
            crate::SyntaxKind::USE_ALIAS => {
                let node = match UseAlias::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::USE_ALIAS into a UseBranchOrAlias::UseAlias")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UseAlias(
                    match node {
                        ::std::borrow::Cow::Owned(owned) => owned,
                        ::std::borrow::Cow::Borrowed(borrowed) => {
                            ::core::clone::Clone::clone(borrowed)
                        }
                    },
                )))
            }
            crate::SyntaxKind::USE_BRANCH => {
                let node = match UseBranch::cast(syntax) {
                    Some(node) => node,
                    None => {
                        if ::core::cfg!(debug_assertions) {
                            :: core :: panic ! ("malformed codegen for casting SyntaxKind::USE_BRANCH into a UseBranchOrAlias::UseBranch")
                        } else {
                            unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                };
                ::core::option::Option::Some(::std::borrow::Cow::Owned(Self::UseBranch(
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
            Self::UseAlias(node) => node.syntax(),
            Self::UseBranch(node) => node.syntax(),
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
pub struct UseDef {
    pub(crate) syntax: crate::SyntaxNode,
}
impl UseDef {
    #[inline]
    pub fn attributes(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Attribute> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn modifiers(&self) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::Modifier> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn keyword(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Use>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn use_tree(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UseTree>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn semicolon(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Semicolon>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for UseDef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_DEF
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
pub struct UseTree {
    pub(crate) syntax: crate::SyntaxNode,
}
impl UseTree {
    #[inline]
    pub fn path(&self) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Path>> {
        crate::ast::support::child(&self.syntax)
    }
    #[inline]
    pub fn use_branch_or_alias(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::UseBranchOrAlias>> {
        crate::ast::support::child(&self.syntax)
    }
}
impl crate::ast::AstNode for UseTree {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_TREE
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
pub struct VarDecl {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VarDecl {
    #[inline]
    pub fn let_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Let>> {
        crate::ast::support::token(&self.syntax)
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
    pub fn type_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::Type>> {
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
    pub fn semicolons(
        &self,
    ) -> crate::ast::support::TokenChildren<'_, crate::ast::tokens::Semicolon> {
        crate::ast::support::token_children(&self.syntax)
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
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
#[repr(transparent)]
pub struct VariantStruct {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VariantStruct {
    #[inline]
    pub fn l_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LCurly>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn fields(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::VariantStructField> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_curly(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RCurly>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantStruct {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_STRUCT
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
pub struct VariantStructField {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VariantStructField {
    #[inline]
    pub fn name(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::nodes::VariantStructFieldName>>
    {
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
impl crate::ast::AstNode for VariantStructField {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_STRUCT_FIELD
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
pub struct VariantStructFieldName {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VariantStructFieldName {
    #[inline]
    pub fn ident(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::Ident>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantStructFieldName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_STRUCT_FIELD_NAME
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
pub struct VariantTuple {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VariantTuple {
    #[inline]
    pub fn l_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::LParen>> {
        crate::ast::support::token(&self.syntax)
    }
    #[inline]
    pub fn variant_tuple_elems(
        &self,
    ) -> crate::ast::support::AstChildren<'_, crate::ast::nodes::VariantTupleElem> {
        crate::ast::support::children(&self.syntax)
    }
    #[inline]
    pub fn r_paren(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::RParen>> {
        crate::ast::support::token(&self.syntax)
    }
}
impl crate::ast::AstNode for VariantTuple {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_TUPLE
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
pub struct VariantTupleElem {
    pub(crate) syntax: crate::SyntaxNode,
}
impl VariantTupleElem {
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
impl crate::ast::AstNode for VariantTupleElem {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_TUPLE_ELEM
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
pub struct WhileExpr {
    pub(crate) syntax: crate::SyntaxNode,
}
impl WhileExpr {
    #[inline]
    pub fn while_token(
        &self,
    ) -> ::core::option::Option<::std::borrow::Cow<'_, crate::ast::tokens::While>> {
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
impl crate::ast::AstNode for WhileExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHILE_EXPR
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
