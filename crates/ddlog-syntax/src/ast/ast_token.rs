use crate::{SyntaxKind, SyntaxToken};
use cstree::TextRange;
use ddlog_diagnostics::{FileId, Interner, Span};

/// Like [`AstNode`][super::AstNode], but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast_from(token: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: &SyntaxToken) -> Option<&Self>;

    fn syntax(&self) -> &SyntaxToken;

    #[inline]
    fn text<'intern>(&self, interner: &'intern Interner) -> &'intern str {
        self.syntax().resolve_text(interner)
    }

    #[inline]
    fn text_range(&self) -> TextRange {
        self.syntax().text_range()
    }

    // FIXME: I don't like having to give the `FileId` here
    #[inline]
    fn span(&self, file: FileId) -> Span {
        let range = self.syntax().text_range();
        Span::new(range.start().into(), range.end().into(), file)
    }
}
