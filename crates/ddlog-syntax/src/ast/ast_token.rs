use crate::{SyntaxKind, SyntaxToken, SyntaxTokenExt};
use cstree::TextRange;
use ddlog_diagnostics::{FileId, Interner, Span};
use std::borrow::Cow;

/// Like [`AstNode`][super::AstNode], but wraps tokens rather than interior nodes.
pub trait AstToken {
    fn can_cast_from(token: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: &SyntaxToken) -> Option<Cow<'_, Self>>
    where
        Self: Clone;

    fn syntax(&self) -> &SyntaxToken;

    #[inline]
    fn text_range(&self) -> TextRange {
        self.syntax().text_range()
    }

    // FIXME: I don't like having to give the `FileId` here
    #[inline]
    fn span(&self, file: FileId) -> Span {
        Span::from_text_range(self.text_range(), file)
    }

    #[inline]
    fn text<'intern>(&self, interner: &'intern Interner) -> &'intern str {
        self.syntax().text(interner)
    }

    #[inline]
    fn lexical_eq(&self, text: &str, interner: &Interner) -> bool {
        self.syntax().lexical_eq(text, interner)
    }
}
