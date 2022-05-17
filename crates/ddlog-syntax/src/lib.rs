#[macro_use]
mod macros;
#[macro_use]
mod syntax_kind;
#[macro_use]
mod token_set;
pub mod ast;
mod lexer;
mod parser;
mod parser_cache;
pub mod queries;
mod syntax;
pub mod validation;
pub mod visitor;

pub use lexer::Token;
pub use syntax::{DifferentialDatalog, SyntaxNodeExt, SyntaxTokenExt};
pub use syntax_kind::SyntaxKind;
pub use token_set::TokenSet;
pub use visitor::{AstVisitor, RuleCtx};

use crate::{
    ast::nodes::Root,
    lexer::Lexer,
    parser::{sink::Sink, Parser},
    SyntaxKind::ROOT,
};
use ddlog_diagnostics::{Diagnostic, FileId, Interner, Span};
use std::mem;

pub type SyntaxNode = cstree::SyntaxNode<DifferentialDatalog>;
pub type GreenNodeBuilder<'cache, 'interner> =
    cstree::GreenNodeBuilder<'cache, 'interner, Interner>;
pub type NodeCache<'interner> = cstree::NodeCache<'interner, Interner>;
pub type SyntaxText<'node, 'interner> =
    cstree::SyntaxText<'node, 'interner, Interner, DifferentialDatalog>;
pub type SyntaxToken = cstree::SyntaxToken<DifferentialDatalog>;
pub type SyntaxElement = cstree::SyntaxElement<DifferentialDatalog>;
pub type SyntaxElementRef<'a> = cstree::SyntaxElementRef<'a, DifferentialDatalog>;

#[inline]
pub fn parse<'interner>(
    file: FileId,
    source: &str,
    cache: NodeCache<'interner>,
) -> (Parsed, NodeCache<'interner>) {
    let interner = cache.interner().clone();
    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let span = Span::single(source.len() as u32, file);

    let (events, errors) = Parser::new(&tokens, span).parse();
    let (root, node_cache) = Sink::new(source, tokens, events, cache).finish();

    let parsed = Parsed::new(SyntaxNode::new_root(root), interner, errors);
    (parsed, node_cache.unwrap())
}

#[inline]
pub fn parse_stmt<'interner>(
    file: FileId,
    source: &str,
    cache: NodeCache<'interner>,
) -> (Parsed, NodeCache<'interner>) {
    let interner = cache.interner().clone();
    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let span = Span::single(source.len() as u32, file);

    let (events, errors) = Parser::new(&tokens, span).parse_stmt();
    let (root, node_cache) = Sink::new(source, tokens, events, cache).finish();

    let parsed = Parsed::new(SyntaxNode::new_root(root), interner, errors);
    (parsed, node_cache.unwrap())
}

#[inline]
pub fn parse_expr<'interner>(
    file: FileId,
    source: &str,
    cache: NodeCache<'interner>,
) -> (Parsed, NodeCache<'interner>) {
    let interner = cache.interner().clone();
    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let span = Span::single(source.len() as u32, file);

    let (events, errors) = Parser::new(&tokens, span).parse_expr();
    let (root, node_cache) = Sink::new(source, tokens, events, cache).finish();

    let parsed = Parsed::new(SyntaxNode::new_root(root), interner, errors);
    (parsed, node_cache.unwrap())
}

#[derive(Debug)]
pub struct Parsed {
    root: SyntaxNode,
    interner: Interner,
    errors: Vec<Diagnostic>,
}

impl Parsed {
    const fn new(root: SyntaxNode, interner: Interner, errors: Vec<Diagnostic>) -> Self {
        Self {
            root,
            interner,
            errors,
        }
    }

    /// Get a reference to the root syntax node
    #[inline]
    pub fn root(&self) -> &Root {
        assert_eq!(self.root.kind(), ROOT);

        // Safety: `SyntaxNode` and `Root` have the same layouts,
        //         `Root` is transparent around `SyntaxNode`
        unsafe { mem::transmute::<&SyntaxNode, &Root>(&self.root) }
    }

    /// Get a reference to the root syntax node
    #[inline]
    pub const fn syntax(&self) -> &SyntaxNode {
        &self.root
    }

    /// Get a reference to the parser's diagnostics
    #[inline]
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }

    /// Return all of the parser's diagnostics and consume the parser
    #[inline]
    pub fn take_errors(&mut self) -> Vec<Diagnostic> {
        mem::take(&mut self.errors)
    }

    /// Return all of the parser's diagnostics and consume the parser
    #[inline]
    pub fn into_errors(self) -> Vec<Diagnostic> {
        self.errors
    }

    /// Get a reference to the current interner
    #[inline]
    pub const fn interner(&self) -> &Interner {
        &self.interner
    }

    /// Returns `true` if any errors occurred while parsing
    #[inline]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    #[inline]
    pub fn into_parts(self) -> (SyntaxNode, Vec<Diagnostic>) {
        (self.root, self.errors)
    }

    /// Make a debug representation of the current tree
    #[inline]
    pub fn debug_tree(&self) -> String {
        let mut debugged = self.root.debug(&self.interner, true);
        // Remove the trailing newline from the debug repr
        debugged.pop();

        debugged
    }
}
