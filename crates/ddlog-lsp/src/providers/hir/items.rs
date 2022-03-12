use crate::{
    database::HirStore,
    providers::hir::{
        BinExpr, BinOp, Expr, FuncDef, FuncParam, HirItem, Literal, Match, MatchArm, Path, Pattern,
        Stmt, Type, VarDecl,
    },
};
use ddlog_diagnostics::{FileId, Interner};
use ddlog_syntax::{
    ast::{
        nodes::{
            Expr as AstExpr, FunctionDef, Item, Literal as AstLiteral, Pattern as AstPattern, Root,
            Stmt as AstStmt, Type as AstType,
        },
        AstNode, AstToken,
    },
    SyntaxNodeExt,
};
use ddlog_utils::ArcSlice;

pub fn hir_items(store: &dyn HirStore, file: FileId) -> ArcSlice<HirItem> {
    let session = store.session();
    let interner = session.interner();
    let uri = file.to_str(interner);
    tracing::debug!(file = uri, "building hir for file");

    let builder = HirBuilder::new(interner);
    let items: Vec<_> = store
        .syntax(file)
        .to::<Root>()
        .items()
        .filter_map(|item| builder.item(&*item))
        .collect();

    let items = ArcSlice::new(items);
    println!("{:#?}", items);
    items
}

struct HirBuilder<'a> {
    interner: &'a Interner,
}

impl<'a> HirBuilder<'a> {
    fn new(interner: &'a Interner) -> Self {
        Self { interner }
    }

    fn item(&self, item: &Item) -> Option<HirItem> {
        match item {
            Item::FunctionDef(func) => self.function(func).map(HirItem::FuncDef),

            Item::ConstDef(_)
            | Item::EnumDef(_)
            | Item::ImplBlock(_)
            | Item::StructDef(_)
            | Item::TypeAlias(_)
            | Item::UseDef(_) => None,
        }
    }

    // TODO: We could make this more resilient, but that would require making the hir more
    //       tolerant to the loss of information. Do this later, once we've figured everything
    //       else out
    fn function(&self, func: &FunctionDef) -> Option<FuncDef> {
        let name = func.ident()?;
        let params = func
            .args()?
            .args()
            .map(|arg| {
                let binding = self.pattern(&*arg.binding()?)?;
                let ty = self.ty(&*arg.ty()?)?;

                Some(FuncParam { binding, ty })
            })
            .collect::<Option<Vec<_>>>()?;

        let body = func
            .body()?
            .statements()
            .filter_map(|stmt| self.stmt(&*stmt))
            .collect::<Vec<_>>();
        //  .map(|stmt| self.stmt(&*stmt))
        //  .collect::<Option<Vec<_>>>()>;

        let return_ty = self.ty(&*func.ret()?.return_ty()?)?;

        Some(FuncDef {
            name,
            params,
            body,
            return_ty,
        })
    }

    fn stmt(&self, stmt: &AstStmt) -> Option<Stmt> {
        match stmt {
            AstStmt::VarDecl(decl) => {
                let binding = self.pattern(&*decl.binding()?)?;
                let value = self.expr(&*decl.value()?)?;

                Some(Stmt::VarDecl(VarDecl { binding, value }))
            }

            // TODO: Semicolon termination or desugaring trailing expressions to a return
            AstStmt::ExprStmt(expr) => self.expr(&*expr.expr()?).map(Stmt::Expr),
        }
    }

    fn expr(&self, expr: &AstExpr) -> Option<Expr> {
        match expr {
            AstExpr::VarRef(var) => var.interned().map(Expr::VarRef),
            AstExpr::RetExpr(ret) => Some(Expr::Return(Box::new(self.expr(&*ret.expr()?)?))),
            AstExpr::Literal(literal) => Some(Expr::Literal(match literal {
                AstLiteral::Bool(bool) => {
                    let bool = if bool.is_true() {
                        true
                    } else if bool.is_false() {
                        false
                    } else {
                        return None;
                    };

                    Literal::Bool(bool)
                }
                AstLiteral::Char(_) => todo!(),
                AstLiteral::Number(number) => {
                    let literal = number.number_literal()?;
                    let number = literal.text(self.interner);

                    Literal::Number(number.parse::<u128>().unwrap())
                }
                AstLiteral::String(_) => todo!(),
            })),

            AstExpr::BinExpr(bin_expr) => {
                let lhs = Box::new(self.expr(&*bin_expr.lhs()?)?);
                let rhs = Box::new(self.expr(&*bin_expr.rhs()?)?);

                let op = bin_expr.op()?;
                let op = if op.is_or() {
                    BinOp::Or
                } else if op.is_and() {
                    BinOp::And
                } else if op.is_plus() {
                    BinOp::Add
                } else if op.is_minus() {
                    BinOp::Sub
                } else if op.is_star() {
                    BinOp::Mul
                } else if op.is_slash() {
                    BinOp::Div
                } else if op.is_eqeq() {
                    BinOp::Eq
                } else if op.is_neq() {
                    BinOp::Neq
                } else if op.is_l_angle() {
                    BinOp::Less
                } else if op.is_l_angle_eq() {
                    BinOp::LessEq
                } else if op.is_r_angle() {
                    BinOp::Greater
                } else if op.is_r_angle_eq() {
                    BinOp::GreaterEq
                } else {
                    return None;
                };

                Some(Expr::BinaryOp(BinExpr { lhs, rhs, op }))
            }

            // We desugar if-else blocks into matches from this
            //
            // ```rust
            // if x {
            //     a
            // } else {
            //     b
            // }
            // ```
            //
            // Into this
            //
            // ```rust
            // match x {
            //     true => a,
            //     false => b,
            // }
            // ```
            //
            // However, if-else statements that include `else if` clauses are
            // desugared differently from this
            //
            // ```rust
            // if x {
            //     a
            // } else if y {
            //     b
            // } else {
            //     c
            // }
            // ```
            //
            // Into this
            //
            // ```rust
            // match () {
            //     () if x => a,
            //     () if y => b,
            //     () => c,
            // }
            // ```
            //
            // TODO: Handle else-if clauses
            AstExpr::IfExpr(if_expr) => {
                let mut ifs = if_expr.if_blocks();
                let if_block = ifs.next()?;

                let scrutinee = Box::new(self.expr(&*if_block.cond()?)?);

                let true_block = if_block.block()?;
                let true_arm = true_block
                    .statements()
                    .map(|stmt| self.stmt(&*stmt))
                    .collect::<Option<Vec<_>>>()?;

                let mut arms = Vec::with_capacity(2);

                // Build the true arm from the if block
                arms.push(MatchArm {
                    binding: Pattern::Literal(Literal::Bool(true)),
                    guard: None,
                    body: Box::new(Expr::Block(true_arm)),
                });

                if let Some(else_block) = if_expr.else_block() {
                    let false_block = else_block.block()?;
                    let false_arm = false_block
                        .statements()
                        .map(|stmt| self.stmt(&*stmt))
                        .collect::<Option<Vec<_>>>()?;

                    // Build the false arm from the else block
                    arms.push(MatchArm {
                        binding: Pattern::Literal(Literal::Bool(false)),
                        guard: None,
                        body: Box::new(Expr::Block(false_arm)),
                    });

                // If no else block is provided, use a unit literal as the false block.
                // This is correct since if there's only one block to the if, the statement
                // must have a unit type overall
                } else {
                    arms.push(MatchArm {
                        binding: Pattern::Literal(Literal::Bool(false)),
                        guard: None,
                        body: Box::new(Expr::Literal(Literal::Unit)),
                    });
                }

                Some(Expr::Match(Match { scrutinee, arms }))
            }

            expr => todo!("{}", expr.syntax().debug(self.interner, true)),
        }
    }

    fn ty(&self, ty: &AstType) -> Option<Type> {
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

    fn pattern(&self, binding: &AstPattern) -> Option<Pattern> {
        match binding {
            AstPattern::VarRef(var) => var.interned().map(Pattern::VarRef),
            AstPattern::StructPattern(_) | AstPattern::TuplePattern(_) => todo!(),
        }
    }
}
