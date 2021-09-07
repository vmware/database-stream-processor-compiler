use crate::{SyntaxNode, SyntaxToken};
use ddlog_diagnostics::{Diagnostic, FileId, Interner, Rope};

/// Context given to a rule when running it.
#[derive(Debug, Clone)]
pub struct RuleCtx {
    /// The file id of the file being linted
    pub file_id: FileId,
    /// An empty vector of diagnostics which the rule adds to
    pub diagnostics: Vec<Diagnostic>,
    pub source: Rope,
    pub interner: Interner,
}

impl RuleCtx {
    pub fn new(file_id: FileId, source: Rope, interner: Interner) -> Self {
        Self {
            file_id,
            diagnostics: Vec::new(),
            source,
            interner,
        }
    }

    pub const fn interner(&self) -> &Interner {
        &self.interner
    }
}

pub trait AstVisitor {
    /// Check an individual node in the syntax tree.
    /// You can use the `match_ast` macro to make matching a node to an ast node easier.
    /// The reason this uses nodes and not a visitor is because nodes are more flexible,
    /// converting them to an AST node has zero cost and you can easily traverse surrounding nodes.
    /// Defaults to doing nothing.
    ///
    /// The return type is [`Option<()>`][`Option`] to allow usage of `?` on the properties of
    /// AST nodes which are all optional.
    #[inline]
    fn check_node(&mut self, _node: &SyntaxNode, _ctx: &mut RuleCtx) -> Option<()> {
        None
    }

    /// Check an individual token in the syntax tree.
    /// Defaults to doing nothing.
    #[inline]
    fn check_token(&mut self, _token: &SyntaxToken, _ctx: &mut RuleCtx) -> Option<()> {
        None
    }

    /// Check the root of the tree one time.
    /// This method is guaranteed to only be called once.
    /// The root's kind will be [`ROOT`][`crate::SyntaxKind::ROOT`].
    /// Defaults to doing nothing.
    #[inline]
    fn check_root(&mut self, _root: &SyntaxNode, _ctx: &mut RuleCtx) -> Option<()> {
        None
    }
}
