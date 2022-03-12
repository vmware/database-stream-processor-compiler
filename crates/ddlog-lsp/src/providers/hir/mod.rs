mod items;

pub(crate) use items::hir_items;

use ddlog_diagnostics::IStr;

// TODO: Arenas

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HirItem {
    FuncDef(FuncDef),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncDef {
    pub name: IStr,
    // generics: Vec<GenericParam>,
    pub params: Vec<FuncParam>,
    pub body: Vec<Stmt>,
    pub return_ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncParam {
    binding: Pattern,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pattern {
    VarRef(IStr),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stmt {
    Expr(Expr),
    VarDecl(VarDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarDecl {
    pub binding: Pattern,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    BinaryOp(BinExpr),
    Match(Match),
    VarRef(IStr),
    Block(Vec<Stmt>),
    Literal(Literal),
    Return(Box<Self>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Unit,
    Bool(bool),
    Number(u128),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Match {
    pub scrutinee: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MatchArm {
    pub binding: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: BinOp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinOp {
    Or,
    And,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Path(Path),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    // TODO: TinyVec?
    pub segments: Vec<IStr>,
}

impl Path {
    pub fn new(segments: Vec<IStr>) -> Self {
        Self { segments }
    }
}
