use crate::{FileId, Span, SyntaxKind};
use logos::Logos;

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
    file: FileId,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(source: &'a str, file: FileId) -> Self {
        Self {
            inner: SyntaxKind::lexer(source),
            file,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        let span = Span::from_range(self.inner.span(), self.file);

        Some(Token::new(kind, span, text))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    kind: SyntaxKind,
    span: Span,
    text: &'a str,
}

impl<'a> Token<'a> {
    pub(crate) const fn new(kind: SyntaxKind, span: Span, text: &'a str) -> Self {
        Self { kind, span, text }
    }

    /// Get the current token's kind
    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    /// Get the current token's range
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Get the current token's text
    pub const fn text(&self) -> &'a str {
        self.text
    }
}
