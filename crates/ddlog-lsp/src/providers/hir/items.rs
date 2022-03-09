use crate::{
    database::HirStore,
    providers::hir::{Expr, FuncDef, FuncParam, HirItem, Path, Pattern, Stmt, Type, VarDecl},
};
use ddlog_diagnostics::FileId;
use ddlog_syntax::{
    ast::{
        nodes::{
            Expr as AstExpr, FunctionDef, Item, Pattern as AstPattern, Root, Stmt as AstStmt,
            Type as AstType,
        },
        AstNode,
    },
    SyntaxNodeExt,
};
use ddlog_utils::ArcSlice;

pub(crate) fn hir_items(store: &dyn HirStore, file: FileId) -> ArcSlice<HirItem> {
    let session = store.session();
    let interner = session.interner();
    let uri = file.to_str(interner);
    tracing::debug!(file = uri, "building hir for file");

    let declarations: Vec<_> = store
        .syntax(file)
        .to::<Root>()
        .items()
        .filter_map(|item| match dbg!(&*item) {
            Item::FunctionDef(func) => {
                println!("{}", func.syntax().debug(interner, true));
                function_hir(func).map(HirItem::FuncDef)
            }

            Item::ConstDef(_)
            | Item::EnumDef(_)
            | Item::ImplBlock(_)
            | Item::StructDef(_)
            | Item::TypeAlias(_)
            | Item::UseDef(_) => None,
        })
        .collect();

    ArcSlice::new(dbg!(declarations))
}

// TODO: We could make this more resilient, but that would require making the hir more
//       tolerant to the loss of information. Do this later, once we've figured everything
//       else out
fn function_hir(func: &FunctionDef) -> Option<FuncDef> {
    let name = func.ident()?;
    let params = func
        .args()?
        .args()
        .map(|arg| {
            let binding = pattern_hir(&*arg.binding()?)?;
            let ty = type_hir(&*arg.ty()?)?;

            Some(FuncParam { binding, ty })
        })
        .collect::<Option<Vec<_>>>()?;

    let body = func
        .body()?
        .statements()
        .filter_map(|stmt| stmt_hir(&*stmt))
        .collect::<Vec<_>>();
    //  .map(|stmt| stmt_hir(&*stmt))
    //  .collect::<Option<Vec<_>>>()>;

    let return_ty = type_hir(&*func.ret()?.return_ty()?)?;

    Some(FuncDef {
        name,
        params,
        body,
        return_ty,
    })
}

fn stmt_hir(stmt: &AstStmt) -> Option<Stmt> {
    match stmt {
        AstStmt::VarDecl(decl) => {
            let binding = pattern_hir(&*decl.binding()?)?;
            let value = expr_hir(&*decl.value()?)?;

            Some(Stmt::VarDecl(VarDecl { binding, value }))
        }

        // TODO: Semicolon termination or desugaring trailing expressions to a return
        AstStmt::ExprStmt(expr) => expr_hir(&*expr.expr()?).map(Stmt::Expr),
    }
}

fn expr_hir(expr: &AstExpr) -> Option<Expr> {
    match expr {
        AstExpr::RetExpr(ret) => Some(Expr::Return(Box::new(expr_hir(&*ret.expr()?)?))),
        AstExpr::VarRef(var) => var.interned().map(Expr::VarRef),
        _ => None,
    }
}

fn type_hir(ty: &AstType) -> Option<Type> {
    match ty {
        AstType::GenericType(generic) => {
            let path = generic.path()?;

            let head = path.head()?.interned();
            let tail = path.tails();

            let mut segments = Vec::with_capacity(1 + tail.len());
            segments.push(head);

            for segment in tail {
                segments.push(segment.tail()?.interned());
            }

            debug_assert!(!segments.is_empty());
            Some(Type::Path(Path::new(segments)))
        }

        AstType::FunctionType(_) | AstType::TupleType(_) => todo!(),
    }
}

fn pattern_hir(binding: &AstPattern) -> Option<Pattern> {
    match binding {
        AstPattern::VarRef(var) => var.interned().map(Pattern::VarRef),
        AstPattern::StructPattern(_) | AstPattern::TuplePattern(_) => todo!(),
    }
}
