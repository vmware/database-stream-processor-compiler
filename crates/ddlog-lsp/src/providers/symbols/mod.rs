mod declarations;

use cstree::interning::Key;
use ddlog_diagnostics::IStr;
use ddlog_syntax::{
    ast::{
        nodes::{Item, Root},
        AstNode,
    },
    match_ast, AstVisitor, RuleCtx, SyntaxNode,
};
use ddlog_utils::ConsistentHasher;
pub(crate) use declarations::declarations;
use std::{collections::HashMap, num::NonZeroU64};

#[derive(Debug, Default)]
struct SymbolResolver(Vec<SyntaxNode>);

impl SymbolResolver {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: SyntaxNode) {
        self.0.push(value);
    }

    pub fn into_inner(self) -> Vec<SyntaxNode> {
        self.0
    }
}

// TODO: Custom debug impl
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TypeId(NonZeroU64);

// TODO: Custom debug impl
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FuncId(NonZeroU64);

// TODO: Custom debug impl
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VarId(NonZeroU64);

// TODO: Structurally hashing values would be a better and more consistent id,
//       taking ideas from Unison <https://www.youtube.com/watch?v=gCWtkvDQ2ZI>
#[derive(Debug)]
pub struct Declarations {
    types: HashMap<TypeId, SyntaxNode, ConsistentHasher>,
    functions: HashMap<FuncId, SyntaxNode, ConsistentHasher>,
    bindings: HashMap<VarId, SyntaxNode, ConsistentHasher>,
    type_id: NonZeroU64,
    func_id: NonZeroU64,
    binding_id: NonZeroU64,
}

impl Declarations {
    pub fn new() -> Self {
        Self {
            types: HashMap::with_hasher(ConsistentHasher::new()),
            functions: HashMap::with_hasher(ConsistentHasher::new()),
            bindings: HashMap::with_hasher(ConsistentHasher::new()),
            type_id: NonZeroU64::new(1).unwrap(),
            func_id: NonZeroU64::new(1).unwrap(),
            binding_id: NonZeroU64::new(1).unwrap(),
        }
    }

    pub fn declare_type(&mut self, ty: SyntaxNode) -> TypeId {
        let id = self.type_id;
        // TODO: Propagate error
        self.type_id =
            unsafe { NonZeroU64::new_unchecked(self.type_id.get().checked_add(1).unwrap()) };
        self.types.insert(TypeId(id), ty);

        TypeId(id)
    }

    pub fn declare_func(&mut self, func: SyntaxNode) -> FuncId {
        let id = self.func_id;
        // TODO: Propagate error
        self.func_id =
            unsafe { NonZeroU64::new_unchecked(self.func_id.get().checked_add(1).unwrap()) };
        self.functions.insert(FuncId(id), func);

        FuncId(id)
    }

    pub fn declare_binding(&mut self, binding: SyntaxNode) -> VarId {
        let id = self.binding_id;
        // TODO: Propagate error
        self.binding_id =
            unsafe { NonZeroU64::new_unchecked(self.binding_id.get().checked_add(1).unwrap()) };
        self.bindings.insert(VarId(id), binding);

        VarId(id)
    }
}

#[derive(Debug)]
pub struct Scope {
    // Invariant: All of these vectors will have at least one map within them
    types: Vec<HashMap<IStr, TypeId, ConsistentHasher>>,
    functions: Vec<HashMap<IStr, FuncId, ConsistentHasher>>,
    bindings: Vec<HashMap<IStr, VarId, ConsistentHasher>>,
    declarations: Declarations,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            types: vec![HashMap::with_hasher(ConsistentHasher::new())],
            functions: vec![HashMap::with_hasher(ConsistentHasher::new())],
            bindings: vec![HashMap::with_hasher(ConsistentHasher::new())],
            declarations: Declarations::new(),
        }
    }

    pub fn declare_type(&mut self, name: IStr, syntax: SyntaxNode) {
        let id = self.declarations.declare_type(syntax);

        // FIXME: Handle double-inserts
        self.types.last_mut().unwrap().insert(name, id);
    }

    pub fn declare_func(&mut self, name: IStr, syntax: SyntaxNode) {
        let id = self.declarations.declare_func(syntax);

        // FIXME: Handle double-inserts
        self.functions.last_mut().unwrap().insert(name, id);
    }

    pub fn declare_binding(&mut self, name: IStr, syntax: SyntaxNode) {
        let id = self.declarations.declare_binding(syntax);

        // FIXME: Handle double-inserts
        self.bindings.last_mut().unwrap().insert(name, id);
    }

    // TODO: Fuzzy string matching for suggestions
    pub fn find_binding(&self, name: IStr) -> Option<VarId> {
        self.bindings
            .iter()
            .rev()
            .find_map(|scope| scope.get(&name).copied())
    }

    pub fn enter_scope<F, T>(&mut self, scope: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        // Push new maps to contain the inner scope
        self.types
            .push(HashMap::with_hasher(ConsistentHasher::new()));
        self.functions
            .push(HashMap::with_hasher(ConsistentHasher::new()));
        self.bindings
            .push(HashMap::with_hasher(ConsistentHasher::new()));

        // Build the inner scope
        let ret = scope(self);

        // Pop the vales from the inner scope
        // TODO: Save these so we can reuse them?
        self.types.pop();
        self.functions.pop();
        self.bindings.pop();

        ret
    }
}

pub enum Hir {
    Func(Func),
}

pub struct Func {
    name: IStr,
    id: FuncId,
    params: Vec<FuncParam>,
    return_type: TypeId,
    body: Block,
    // generics: Vec<GenericParam>,
}

pub struct FuncParam {
    name: IStr,
    id: VarId,
    ty: TypeId,
}

pub struct Block {
    stmts: Vec<Stmt>,
}

pub enum Stmt {}

// pub struct GenericParam {
//     name: IStr,
//     // TODO: Bounds
// }

impl AstVisitor for SymbolResolver {
    // TODO: Probably want to be more incremental about this
    // TODO: Use im for maps
    fn check_root(&mut self, root: &Root, ctx: &mut RuleCtx) -> Option<()> {
        let mut scope = Scope::new();

        // Perform an initial pass to collect top-level definitions
        for item in root.items() {
            match &*item {
                Item::ConstDef(const_def) => {
                    if let Some(name) = const_def.ident() {
                        scope.declare_binding(name, const_def.syntax().clone());
                    }
                }

                // Functions are special, they're declared both as a function declaration and as a
                // variable bound within the outer scope. This allows functions to be both called
                // unambiguously and used as function references
                Item::FunctionDef(func_def) => {
                    if let Some(name) = func_def.ident() {
                        scope.declare_func(name, func_def.syntax().clone());
                        scope.declare_binding(name, func_def.syntax().clone());
                    }
                }

                Item::EnumDef(enum_def) => {
                    if let Some(name) = enum_def.ident() {
                        scope.declare_type(name, enum_def.syntax().clone());
                    }
                }
                Item::StructDef(struct_def) => {
                    if let Some(name) = struct_def.ident() {
                        scope.declare_type(name, struct_def.syntax().clone());
                    }
                }
                Item::TypeAlias(_) => {}

                Item::ImplBlock(_) => {}
                Item::UseDef(_) => {}
            }
        }

        // Resolve the bodies and sub-elements of all top-level definitions
        for item in root.items() {
            match &*item {
                Item::FunctionDef(func_def) => {
                    scope.enter_scope(|scope| {
                        if let Some(args) = func_def.args() {
                            for arg in args.args() {
                                // - Find all bindings
                                // - Unfold patterns and their corresponding types at the same time
                                //   in order to get the types of pattern matched bindings
                                // - Resolve argument types
                                // - Check that all pattern matched fields and all inner types exist
                            }
                        }
                    });
                }

                Item::EnumDef(_) => {}
                Item::StructDef(_) => {}
                Item::ConstDef(_) => {}
                Item::TypeAlias(_) => {}
                Item::ImplBlock(_) => {}
                Item::UseDef(_) => {}
            }
        }

        println!("{:?}", scope);

        None
    }
}
