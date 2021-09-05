use crate::{SyntaxNode, SyntaxToken};
use ddlog_diagnostics::{Diagnostic, Rope};

/// Context given to a rule when running it.
#[derive(Debug, Clone)]
pub struct RuleCtx {
    /// The file id of the file being linted
    pub file_id: usize,
    /// Whether the linter is run with the `--verbose` option
    /// Which dictates whether the linter should include more (potentially spammy) context in diagnostics
    pub verbose: bool,
    /// An empty vector of diagnostics which the rule adds to
    pub diagnostics: Vec<Diagnostic>,
    pub source: Rope,
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
