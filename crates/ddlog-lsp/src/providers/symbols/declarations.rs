use crate::{database::Symbols, providers::symbols::SymbolResolver};
use ddlog_diagnostics::FileId;
use ddlog_syntax::{match_ast, visitor, AstVisitor, RuleCtx, SyntaxNode};
use ddlog_utils::ArcSlice;

pub(crate) fn declarations(symbols: &dyn Symbols, file: FileId) -> ArcSlice<SyntaxNode> {
    let session = symbols.session();
    let interner = session.interner();
    let uri = file.to_str(interner);

    tracing::debug!(file = uri, "collecting declarations for a file");

    let root = symbols.syntax(file);
    let mut ctx = RuleCtx::new(file, symbols.file_source(file), interner.clone());

    let mut collector = DeclarationCollector::new();
    visitor::apply_visitor(&root, &mut collector, &mut ctx);

    let declarations = collector.into_inner();
    tracing::debug!(
        file = uri,
        "collected {} total declarations for",
        declarations.len(),
    );

    let mut collector = SymbolResolver::new();
    visitor::apply_visitor(&root, &mut collector, &mut ctx);

    ArcSlice::new(declarations)
}

#[derive(Debug, Default)]
struct DeclarationCollector(Vec<SyntaxNode>);

impl DeclarationCollector {
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

impl AstVisitor for DeclarationCollector {
    fn check_node(&mut self, node: &SyntaxNode, _ctx: &mut RuleCtx) -> Option<()> {
        match_ast! {
            match node {
                EnumDef(_) => self.push(node.clone()),
                ConstDef(_) => self.push(node.clone()),
                StructDef(_) => self.push(node.clone()),
                FunctionDef(_) => self.push(node.clone()),
                // TODO: Type definitions
                // TODO: Type aliases

                _ => {},
            }
        }

        None
    }
}
