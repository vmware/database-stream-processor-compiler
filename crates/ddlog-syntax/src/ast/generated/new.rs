/// Ampersand, ampersand, AMPERSAND, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Ampersand {
    syntax: crate::SyntaxToken,
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
/// AmpersandEq, ampersand_eq, AMPERSAND_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct AmpersandEq {
    syntax: crate::SyntaxToken,
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
/// And, and_token, AND, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct And {
    syntax: crate::SyntaxToken,
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
/// ArrayAccess, array_access, ARRAY_ACCESS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ArrayAccess {
    syntax: crate::SyntaxNode,
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
/// ArrayExprElem, array_expr_elem, ARRAY_EXPR_ELEM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ArrayExprElem {
    syntax: crate::SyntaxNode,
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
/// ArrayInitExpr, array_init_expr, ARRAY_INIT_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ArrayInitExpr {
    syntax: crate::SyntaxNode,
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
/// As, as_token, AS, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct As {
    syntax: crate::SyntaxToken,
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
/// Assign, assign, ASSIGN, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Assign {
    syntax: crate::SyntaxNode,
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
/// AttrName, attr_name, ATTR_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct AttrName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for AttrName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ATTR_NAME
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
/// AttrPair, attr_pair, ATTR_PAIR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct AttrPair {
    syntax: crate::SyntaxNode,
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
/// Attribute, attribute, ATTRIBUTE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Attribute {
    syntax: crate::SyntaxNode,
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
/// Bang, bang, BANG, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Bang {
    syntax: crate::SyntaxToken,
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
/// BinExpr, bin_expr, BIN_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct BinExpr {
    syntax: crate::SyntaxNode,
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
/// Block, block, BLOCK, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Block {
    syntax: crate::SyntaxNode,
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
/// BrackedStructField, bracked_struct_field, BRACKED_STRUCT_FIELD, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct BrackedStructField {
    syntax: crate::SyntaxNode,
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
/// BrackedStructFields, bracked_struct_fields, BRACKED_STRUCT_FIELDS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct BrackedStructFields {
    syntax: crate::SyntaxNode,
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
/// Break, break_token, BREAK, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Break {
    syntax: crate::SyntaxToken,
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
/// BreakExpr, break_expr, BREAK_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct BreakExpr {
    syntax: crate::SyntaxNode,
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
/// Caret, caret, CARET, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Caret {
    syntax: crate::SyntaxToken,
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
/// CaretEq, caret_eq, CARET_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct CaretEq {
    syntax: crate::SyntaxToken,
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
/// ClosureArg, closure_arg, CLOSURE_ARG, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ClosureArg {
    syntax: crate::SyntaxNode,
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
/// ClosureExpr, closure_expr, CLOSURE_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ClosureExpr {
    syntax: crate::SyntaxNode,
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
/// Colon, colon, COLON, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Colon {
    syntax: crate::SyntaxToken,
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
/// Comma, comma, COMMA, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Comma {
    syntax: crate::SyntaxToken,
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
/// Comment, comment, COMMENT, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Comment {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Comment {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::COMMENT
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
/// Const, const_token, CONST, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Const {
    syntax: crate::SyntaxToken,
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
/// ConstDef, const_def, CONST_DEF, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ConstDef {
    syntax: crate::SyntaxNode,
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
/// ConstName, const_name, CONST_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ConstName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ConstName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONST_NAME
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
/// ConstValue, const_value, CONST_VALUE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ConstValue {
    syntax: crate::SyntaxNode,
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
/// Continue, continue, CONTINUE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Continue {
    syntax: crate::SyntaxToken,
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
/// ContinueExpr, continue_expr, CONTINUE_EXPR, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ContinueExpr {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for ContinueExpr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::CONTINUE_EXPR
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
/// Dot, dot, DOT, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Dot {
    syntax: crate::SyntaxToken,
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
/// DoubleColon, double_colon, DOUBLE_COLON, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct DoubleColon {
    syntax: crate::SyntaxToken,
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
/// Else, else_token, ELSE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Else {
    syntax: crate::SyntaxToken,
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
/// ElseBlock, else_block, ELSE_BLOCK, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ElseBlock {
    syntax: crate::SyntaxNode,
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
/// Enum, enum_token, ENUM, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Enum {
    syntax: crate::SyntaxToken,
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
/// EnumDef, enum_def, ENUM_DEF, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct EnumDef {
    syntax: crate::SyntaxNode,
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
/// EnumName, enum_name, ENUM_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct EnumName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for EnumName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_NAME
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
/// EnumVariant, enum_variant, ENUM_VARIANT, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct EnumVariant {
    syntax: crate::SyntaxNode,
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
/// EnumVariantName, enum_variant_name, ENUM_VARIANT_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct EnumVariantName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for EnumVariantName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANT_NAME
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
/// EnumVariants, enum_variants, ENUM_VARIANTS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct EnumVariants {
    syntax: crate::SyntaxNode,
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
/// Eof, eof, EOF, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Eof {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Eof {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EOF
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
/// Eq, eq, EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Eq {
    syntax: crate::SyntaxToken,
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
/// Eqeq, eqeq, EQEQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Eqeq {
    syntax: crate::SyntaxToken,
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
/// ExprStmt, expr_stmt, EXPR_STMT, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ExprStmt {
    syntax: crate::SyntaxNode,
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
/// False, false_token, FALSE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct False {
    syntax: crate::SyntaxToken,
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
/// FieldAccess, field_access, FIELD_ACCESS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FieldAccess {
    syntax: crate::SyntaxNode,
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
/// FieldAccessorName, field_accessor_name, FIELD_ACCESSOR_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FieldAccessorName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for FieldAccessorName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FIELD_ACCESSOR_NAME
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
/// Fn, fn, FN, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Fn {
    syntax: crate::SyntaxToken,
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
/// For, for_token, FOR, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct For {
    syntax: crate::SyntaxToken,
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
/// ForExpr, for_expr, FOR_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ForExpr {
    syntax: crate::SyntaxNode,
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
/// FunctionArg, function_arg, FUNCTION_ARG, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionArg {
    syntax: crate::SyntaxNode,
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
/// FunctionArgs, function_args, FUNCTION_ARGS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionArgs {
    syntax: crate::SyntaxNode,
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
/// FunctionCall, function_call, FUNCTION_CALL, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionCall {
    syntax: crate::SyntaxNode,
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
/// FunctionCallArg, function_call_arg, FUNCTION_CALL_ARG, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionCallArg {
    syntax: crate::SyntaxNode,
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
/// FunctionDef, function_def, FUNCTION_DEF, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionDef {
    syntax: crate::SyntaxNode,
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
/// FunctionName, function_name, FUNCTION_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for FunctionName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FUNCTION_NAME
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
/// FunctionReturn, function_return, FUNCTION_RETURN, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionReturn {
    syntax: crate::SyntaxNode,
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
/// FunctionReturnType, function_return_type, FUNCTION_RETURN_TYPE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionReturnType {
    syntax: crate::SyntaxNode,
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
/// FunctionType, function_type, FUNCTION_TYPE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionType {
    syntax: crate::SyntaxNode,
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
/// FunctionTypeArg, function_type_arg, FUNCTION_TYPE_ARG, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionTypeArg {
    syntax: crate::SyntaxNode,
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
/// FunctionTypeArgs, function_type_args, FUNCTION_TYPE_ARGS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct FunctionTypeArgs {
    syntax: crate::SyntaxNode,
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
/// GenericArg, generic_arg, GENERIC_ARG, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct GenericArg {
    syntax: crate::SyntaxNode,
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
/// GenericType, generic_type, GENERIC_TYPE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct GenericType {
    syntax: crate::SyntaxNode,
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
/// Generics, generics, GENERICS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Generics {
    syntax: crate::SyntaxNode,
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
/// HashBrack, hash_brack, HASH_BRACK, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct HashBrack {
    syntax: crate::SyntaxToken,
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
/// Ident, ident, IDENT, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Ident {
    syntax: crate::SyntaxToken,
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
/// If, if_token, IF, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct If {
    syntax: crate::SyntaxToken,
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
/// IfBlock, if_block, IF_BLOCK, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct IfBlock {
    syntax: crate::SyntaxNode,
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
/// IfExpr, if_expr, IF_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct IfExpr {
    syntax: crate::SyntaxNode,
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
/// Impl, impl, IMPL, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Impl {
    syntax: crate::SyntaxToken,
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
/// ImplBlock, impl_block, IMPL_BLOCK, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ImplBlock {
    syntax: crate::SyntaxNode,
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
/// ImplBlockContents, impl_block_contents, IMPL_BLOCK_CONTENTS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ImplBlockContents {
    syntax: crate::SyntaxNode,
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
/// In, in, IN, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct In {
    syntax: crate::SyntaxToken,
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
/// LAngle, l_angle, L_ANGLE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct LAngle {
    syntax: crate::SyntaxToken,
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
/// LAngleEq, l_angle_eq, L_ANGLE_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct LAngleEq {
    syntax: crate::SyntaxToken,
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
/// LBrack, l_brack, L_BRACK, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct LBrack {
    syntax: crate::SyntaxToken,
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
/// LCurly, l_curly, L_CURLY, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct LCurly {
    syntax: crate::SyntaxToken,
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
/// LParen, l_paren, L_PAREN, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct LParen {
    syntax: crate::SyntaxToken,
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
/// Let, let_token, LET, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Let {
    syntax: crate::SyntaxToken,
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
/// Loop, loop_token, LOOP, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Loop {
    syntax: crate::SyntaxToken,
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
/// LoopExpr, loop_expr, LOOP_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct LoopExpr {
    syntax: crate::SyntaxNode,
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
/// Match, match_token, MATCH, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Match {
    syntax: crate::SyntaxToken,
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
/// MatchArm, match_arm, MATCH_ARM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct MatchArm {
    syntax: crate::SyntaxNode,
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
/// MatchExpr, match_expr, MATCH_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct MatchExpr {
    syntax: crate::SyntaxNode,
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
/// Minus, minus, MINUS, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Minus {
    syntax: crate::SyntaxToken,
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
/// MinusEq, minus_eq, MINUS_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct MinusEq {
    syntax: crate::SyntaxToken,
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
/// Modifier, modifier, MODIFIER, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Modifier {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Modifier {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::MODIFIER
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
/// Neq, neq, NEQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Neq {
    syntax: crate::SyntaxToken,
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
/// Number, number, NUMBER, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Number {
    syntax: crate::SyntaxToken,
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
/// Number, number, NUMBER, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Number {
    syntax: crate::SyntaxToken,
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
/// Or, or_token, OR, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Or {
    syntax: crate::SyntaxToken,
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
/// ParenExpr, paren_expr, PAREN_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ParenExpr {
    syntax: crate::SyntaxNode,
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
/// Path, path, PATH, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Path {
    syntax: crate::SyntaxNode,
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
/// PathSegment, path_segment, PATH_SEGMENT, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct PathSegment {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for PathSegment {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATH_SEGMENT
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
/// PathTail, path_tail, PATH_TAIL, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct PathTail {
    syntax: crate::SyntaxNode,
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
/// Percent, percent, PERCENT, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Percent {
    syntax: crate::SyntaxToken,
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
/// PercentEq, percent_eq, PERCENT_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct PercentEq {
    syntax: crate::SyntaxToken,
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
/// Pipe, pipe, PIPE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Pipe {
    syntax: crate::SyntaxToken,
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
/// PipeEq, pipe_eq, PIPE_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct PipeEq {
    syntax: crate::SyntaxToken,
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
/// Plus, plus, PLUS, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Plus {
    syntax: crate::SyntaxToken,
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
/// PlusEq, plus_eq, PLUS_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct PlusEq {
    syntax: crate::SyntaxToken,
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
/// Pub, pub, PUB, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Pub {
    syntax: crate::SyntaxToken,
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
/// QualifiedRef, qualified_ref, QUALIFIED_REF, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct QualifiedRef {
    syntax: crate::SyntaxNode,
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
/// RAngle, r_angle, R_ANGLE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RAngle {
    syntax: crate::SyntaxToken,
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
/// RAngleEq, r_angle_eq, R_ANGLE_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RAngleEq {
    syntax: crate::SyntaxToken,
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
/// RBrack, r_brack, R_BRACK, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RBrack {
    syntax: crate::SyntaxToken,
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
/// RCurly, r_curly, R_CURLY, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RCurly {
    syntax: crate::SyntaxToken,
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
/// RParen, r_paren, R_PAREN, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RParen {
    syntax: crate::SyntaxToken,
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
/// RetExpr, ret_expr, RET_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RetExpr {
    syntax: crate::SyntaxNode,
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
/// Return, return_token, RETURN, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Return {
    syntax: crate::SyntaxToken,
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
/// RightArrow, right_arrow, RIGHT_ARROW, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RightArrow {
    syntax: crate::SyntaxToken,
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
/// RightRocket, right_rocket, RIGHT_ROCKET, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct RightRocket {
    syntax: crate::SyntaxToken,
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
/// Root, root, ROOT, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Root {
    syntax: crate::SyntaxNode,
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
/// Semicolon, semicolon, SEMICOLON, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Semicolon {
    syntax: crate::SyntaxToken,
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
/// Shl, shl, SHL, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Shl {
    syntax: crate::SyntaxToken,
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
/// ShlEq, shl_eq, SHL_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ShlEq {
    syntax: crate::SyntaxToken,
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
/// Shr, shr, SHR, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Shr {
    syntax: crate::SyntaxToken,
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
/// ShrEq, shr_eq, SHR_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct ShrEq {
    syntax: crate::SyntaxToken,
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
/// Slash, slash, SLASH, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Slash {
    syntax: crate::SyntaxToken,
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
/// SlashEq, slash_eq, SLASH_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct SlashEq {
    syntax: crate::SyntaxToken,
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
/// Star, star, STAR, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Star {
    syntax: crate::SyntaxToken,
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
/// StarEq, star_eq, STAR_EQ, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StarEq {
    syntax: crate::SyntaxToken,
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
/// String, string, STRING, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct String {
    syntax: crate::SyntaxToken,
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
/// String, string, STRING, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct String {
    syntax: crate::SyntaxToken,
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
/// Struct, struct_token, STRUCT, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Struct {
    syntax: crate::SyntaxToken,
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
/// StructDef, struct_def, STRUCT_DEF, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructDef {
    syntax: crate::SyntaxNode,
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
/// StructFieldName, struct_field_name, STRUCT_FIELD_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructFieldName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StructFieldName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_FIELD_NAME
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
/// StructInitExpr, struct_init_expr, STRUCT_INIT_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructInitExpr {
    syntax: crate::SyntaxNode,
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
/// StructInitField, struct_init_field, STRUCT_INIT_FIELD, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructInitField {
    syntax: crate::SyntaxNode,
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
/// StructName, struct_name, STRUCT_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StructName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_NAME
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
/// StructPattern, struct_pattern, STRUCT_PATTERN, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructPattern {
    syntax: crate::SyntaxNode,
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
/// StructPatternField, struct_pattern_field, STRUCT_PATTERN_FIELD, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructPatternField {
    syntax: crate::SyntaxNode,
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
/// StructPatternFieldName, struct_pattern_field_name, STRUCT_PATTERN_FIELD_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct StructPatternFieldName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for StructPatternFieldName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_PATTERN_FIELD_NAME
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
/// Tombstone, tombstone, TOMBSTONE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Tombstone {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Tombstone {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::TOMBSTONE
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
/// True, true_token, TRUE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct True {
    syntax: crate::SyntaxToken,
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
/// TupleExprElem, tuple_expr_elem, TUPLE_EXPR_ELEM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TupleExprElem {
    syntax: crate::SyntaxNode,
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
/// TupleInitExpr, tuple_init_expr, TUPLE_INIT_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TupleInitExpr {
    syntax: crate::SyntaxNode,
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
/// TuplePattern, tuple_pattern, TUPLE_PATTERN, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TuplePattern {
    syntax: crate::SyntaxNode,
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
/// TuplePatternElem, tuple_pattern_elem, TUPLE_PATTERN_ELEM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TuplePatternElem {
    syntax: crate::SyntaxNode,
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
/// TupleStructField, tuple_struct_field, TUPLE_STRUCT_FIELD, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TupleStructField {
    syntax: crate::SyntaxNode,
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
/// TupleStructFields, tuple_struct_fields, TUPLE_STRUCT_FIELDS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TupleStructFields {
    syntax: crate::SyntaxNode,
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
/// TupleType, tuple_type, TUPLE_TYPE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TupleType {
    syntax: crate::SyntaxNode,
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
/// TupleTypeElem, tuple_type_elem, TUPLE_TYPE_ELEM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TupleTypeElem {
    syntax: crate::SyntaxNode,
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
/// Type, type_token, TYPE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Type {
    syntax: crate::SyntaxToken,
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
/// TypeAlias, type_alias, TYPE_ALIAS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct TypeAlias {
    syntax: crate::SyntaxNode,
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
/// UnaryExpr, unary_expr, UNARY_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct UnaryExpr {
    syntax: crate::SyntaxNode,
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
/// Use, use, USE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Use {
    syntax: crate::SyntaxToken,
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
/// UseAlias, use_alias, USE_ALIAS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct UseAlias {
    syntax: crate::SyntaxNode,
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
/// UseAliasName, use_alias_name, USE_ALIAS_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct UseAliasName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for UseAliasName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_ALIAS_NAME
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
/// UseBranch, use_branch, USE_BRANCH, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct UseBranch {
    syntax: crate::SyntaxNode,
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
/// UseDef, use_def, USE_DEF, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct UseDef {
    syntax: crate::SyntaxNode,
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
/// UseTree, use_tree, USE_TREE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct UseTree {
    syntax: crate::SyntaxNode,
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
/// VarDecl, var_decl, VAR_DECL, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VarDecl {
    syntax: crate::SyntaxNode,
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
/// VarRef, var_ref, VAR_REF, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VarRef {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for VarRef {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VAR_REF
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
/// VariantStruct, variant_struct, VARIANT_STRUCT, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VariantStruct {
    syntax: crate::SyntaxNode,
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
/// VariantStructField, variant_struct_field, VARIANT_STRUCT_FIELD, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VariantStructField {
    syntax: crate::SyntaxNode,
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
/// VariantStructFieldName, variant_struct_field_name, VARIANT_STRUCT_FIELD_NAME, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VariantStructFieldName {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for VariantStructFieldName {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::VARIANT_STRUCT_FIELD_NAME
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
/// VariantTuple, variant_tuple, VARIANT_TUPLE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VariantTuple {
    syntax: crate::SyntaxNode,
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
/// VariantTupleElem, variant_tuple_elem, VARIANT_TUPLE_ELEM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct VariantTupleElem {
    syntax: crate::SyntaxNode,
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
/// While, while_token, WHILE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct While {
    syntax: crate::SyntaxToken,
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
/// WhileExpr, while_expr, WHILE_EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct WhileExpr {
    syntax: crate::SyntaxNode,
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
/// Whitespace, whitespace, WHITESPACE, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub struct Whitespace {
    syntax: crate::SyntaxToken,
}
impl crate::ast::AstToken for Whitespace {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::WHITESPACE
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
/// AssignOp, assign_op, ASSIGN_OP, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum AssignOp {
    Eq(Eq),
    PlusEq(PlusEq),
    MinusEq(MinusEq),
    SlashEq(SlashEq),
    StarEq(StarEq),
    PercentEq(PercentEq),
    AmpersandEq(AmpersandEq),
    PipeEq(PipeEq),
    CaretEq(CaretEq),
    ShlEq(ShlEq),
    ShrEq(ShrEq),
}
impl crate::ast::AstToken for AssignOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ASSIGN_OP
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
/// BinOp, bin_op, BIN_OP, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum BinOp {
    Plus(Plus),
    Minus(Minus),
    Star(Star),
    Slash(Slash),
    Percent(Percent),
    Pipe(Pipe),
    Caret(Caret),
    Ampersand(Ampersand),
    Shl(Shl),
    Shr(Shr),
    And(And),
    Or(Or),
    Eqeq(Eqeq),
    Neq(Neq),
    RAngle(RAngle),
    RAngleEq(RAngleEq),
    LAngle(LAngle),
    LAngleEq(LAngleEq),
}
impl crate::ast::AstToken for BinOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BIN_OP
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
/// Bool, bool, BOOL, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Bool {
    True(True),
    False(False),
}
impl crate::ast::AstToken for Bool {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::BOOL
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
/// EnumVariantBody, enum_variant_body, ENUM_VARIANT_BODY, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum EnumVariantBody {
    VariantTuple(VariantTuple),
    VariantStruct(VariantStruct),
}
impl crate::ast::AstNode for EnumVariantBody {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ENUM_VARIANT_BODY
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
/// Expr, expr, EXPR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Expr {
    Literal(Literal),
    VarRef(VarRef),
    QualifiedRef(QualifiedRef),
    Assign(Assign),
    ParenExpr(ParenExpr),
    BinExpr(BinExpr),
    IfExpr(IfExpr),
    RetExpr(RetExpr),
    BreakExpr(BreakExpr),
    ContinueExpr(ContinueExpr),
    UnaryExpr(UnaryExpr),
    Block(Block),
    WhileExpr(WhileExpr),
    ForExpr(ForExpr),
    LoopExpr(LoopExpr),
    MatchExpr(MatchExpr),
    ClosureExpr(ClosureExpr),
    FieldAccess(FieldAccess),
    ArrayAccess(ArrayAccess),
    FunctionCall(FunctionCall),
    StructInitExpr(StructInitExpr),
    TupleInitExpr(TupleInitExpr),
    ArrayInitExpr(ArrayInitExpr),
}
impl crate::ast::AstNode for Expr {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::EXPR
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
/// FieldAccessor, field_accessor, FIELD_ACCESSOR, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum FieldAccessor {
    FieldAccessorName(FieldAccessorName),
    Number(Number),
}
impl crate::ast::AstNode for FieldAccessor {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::FIELD_ACCESSOR
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
/// Item, item, ITEM, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Item {
    FunctionDef(FunctionDef),
    StructDef(StructDef),
    EnumDef(EnumDef),
    ConstDef(ConstDef),
    UseDef(UseDef),
    ImplBlock(ImplBlock),
    TypeAlias(TypeAlias),
}
impl crate::ast::AstNode for Item {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::ITEM
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
/// Literal, literal, LITERAL, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Literal {
    Bool(Bool),
    Number(Number),
    String(String),
}
impl crate::ast::AstNode for Literal {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::LITERAL
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
/// Pattern, pattern, PATTERN, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Pattern {
    VarRef(VarRef),
    TuplePattern(TuplePattern),
    StructPattern(StructPattern),
}
impl crate::ast::AstNode for Pattern {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::PATTERN
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
/// Stmt, stmt, STMT, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Stmt {
    ExprStmt(ExprStmt),
    VarDecl(VarDecl),
}
impl crate::ast::AstNode for Stmt {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STMT
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
/// StructFields, struct_fields, STRUCT_FIELDS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum StructFields {
    BrackedStructFields(BrackedStructFields),
    TupleStructFields(TupleStructFields),
}
impl crate::ast::AstNode for StructFields {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::STRUCT_FIELDS
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
/// Type, type, TYPE, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum Type {
    GenericType(GenericType),
    TupleType(TupleType),
    FunctionType(FunctionType),
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
/// UnaryOp, unary_op, UNARY_OP, Token
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum UnaryOp {
    Bang(Bang),
    Minus(Minus),
}
impl crate::ast::AstToken for UnaryOp {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::UNARY_OP
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
/// UseBranchOrAlias, use_branch_or_alias, USE_BRANCH_OR_ALIAS, Syntax
#[derive(
    :: core :: fmt :: Debug,
    :: core :: clone :: Clone,
    :: core :: cmp :: PartialEq,
    :: core :: cmp :: Eq,
    :: core :: hash :: Hash,
)]
pub enum UseBranchOrAlias {
    UseBranch(UseBranch),
    UseAlias(UseAlias),
}
impl crate::ast::AstNode for UseBranchOrAlias {
    #[inline]
    fn can_cast_from(kind: crate::SyntaxKind) -> bool {
        kind == crate::SyntaxKind::USE_BRANCH_OR_ALIAS
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
