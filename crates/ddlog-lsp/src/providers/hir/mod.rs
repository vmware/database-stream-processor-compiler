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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stmt {
    VarDecl(VarDecl),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarDecl {
    pub binding: Pattern,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expr {
    VarRef(IStr),
    Return(Box<Self>),
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
