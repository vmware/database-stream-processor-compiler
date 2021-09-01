#[macro_use]
mod syntax_kind;
#[macro_use]
mod token_set;
mod lexer;
mod parser;
mod syntax;

pub use lexer::Token;
pub use syntax::DifferentialDatalog;
pub use syntax_kind::SyntaxKind;
pub use token_set::TokenSet;

use crate::{
    lexer::Lexer,
    parser::{sink::Sink, Parser},
};
use ddlog_diagnostics::{Diagnostic, FileId, Interner, Span};

pub type SyntaxNode = cstree::SyntaxNode<DifferentialDatalog>;
pub type GreenNodeBuilder<'cache, 'interner> =
    cstree::GreenNodeBuilder<'cache, 'interner, Interner>;
pub type NodeCache<'interner> = cstree::NodeCache<'interner, Interner>;

#[inline]
pub fn parse(file: FileId, source: &str, cache: &mut NodeCache<'_>) -> Parsed {
    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let span = Span::single(source.len() as u32, file);
    let (events, errors) = Parser::new(&tokens, span).parse();
    let root = Sink::new(source, tokens, events, cache).finish();

    Parsed::new(SyntaxNode::new_root(root), cache.interner().clone(), errors)
}

#[inline]
pub fn parse_expr(file: FileId, source: &str, cache: &mut NodeCache<'_>) -> Parsed {
    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let span = Span::single(source.len() as u32, file);
    let (events, errors) = Parser::new(&tokens, span).parse_expr();
    let root = Sink::new(source, tokens, events, cache).finish();

    Parsed::new(SyntaxNode::new_root(root), cache.interner().clone(), errors)
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
    pub const fn root(&self) -> &SyntaxNode {
        &self.root
    }

    /// Get a reference to the parser's diagnostics
    #[inline]
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
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

    /// Make a debug representation of the current tree
    #[inline]
    pub fn debug_tree(&self) -> String {
        let mut debugged = self.root.debug(&self.interner, true);
        // Remove the trailing newline from the debug repr
        debugged.pop();

        debugged
    }
}
