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

pub fn parse(
    file: FileId,
    source: &str,
    cache: &mut NodeCache<'_>,
) -> (SyntaxNode, Vec<Diagnostic>) {
    let span = Span::new(
        source.len().saturating_sub(1) as u32,
        source.len() as u32,
        file,
    );

    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let (events, errors) = Parser::new(&tokens, span).parse();
    let root = Sink::new(source, tokens, events, cache).finish();

    (SyntaxNode::new_root(root), errors)
}

pub fn parse_expr(
    file: FileId,
    source: &str,
    cache: &mut NodeCache<'_>,
) -> (SyntaxNode, Vec<Diagnostic>) {
    let span = Span::new(
        source.len().saturating_sub(1) as u32,
        source.len() as u32,
        file,
    );

    let tokens: Vec<_> = Lexer::new(source, file).collect();
    let (events, errors) = Parser::new(&tokens, span).parse_expr();
    let root = Sink::new(source, tokens, events, cache).finish();

    (SyntaxNode::new_root(root), errors)
}
