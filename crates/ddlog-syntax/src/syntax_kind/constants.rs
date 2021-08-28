use crate::SyntaxKind;

pub const BINOP: SyntaxKind = SyntaxKind::BinaryExpr;
pub const PREFIX: SyntaxKind = SyntaxKind::PrefixExpr;
pub const PARENS: SyntaxKind = SyntaxKind::ParenExpr;
pub const LITERAL: SyntaxKind = SyntaxKind::Literal;
pub const NAME_REF: SyntaxKind = SyntaxKind::NameRef;
pub const FUNC_DECL: SyntaxKind = SyntaxKind::FnDecl;
pub const FUNC_NAME: SyntaxKind = SyntaxKind::FnName;
pub const FUNC_ARGS: SyntaxKind = SyntaxKind::FnArgs;
pub const FUNC_BODY: SyntaxKind = SyntaxKind::FnBody;
