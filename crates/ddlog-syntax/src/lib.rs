#[macro_use]
mod syntax_kind;
pub mod hasher;
mod interner;
mod lexer;
mod parser;
mod span;
mod syntax;

pub use interner::Interner;
pub use lexer::Token;
pub use span::Span;
pub use syntax::DifferentialDatalog;
pub use syntax_kind::{constants, SyntaxKind};

use crate::{
    lexer::Lexer,
    parser::{sink::Sink, Parser},
};
use ariadne::Report;
use lasso::Spur;

pub type SyntaxNode = cstree::SyntaxNode<DifferentialDatalog>;
pub type GreenNodeBuilder<'cache, 'interner> =
    cstree::GreenNodeBuilder<'cache, 'interner, Interner>;
pub type NodeCache<'interner> = cstree::NodeCache<'interner, Interner>;

pub fn parse(
    file: FileId,
    source: &str,
    cache: &mut NodeCache<'_>,
) -> (SyntaxNode, Vec<Report<Span>>) {
    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let (events, errors) = Parser::new(&tokens, Span::single(source.len() as u32, file)).parse();
    let root = Sink::new(source, tokens, events, cache).finish();

    (SyntaxNode::new_root(root), errors)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FileId(Spur);

impl FileId {
    pub const fn new(path: Spur) -> Self {
        Self(path)
    }
}
