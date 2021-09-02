use crate::{SyntaxKind, SyntaxNode, SyntaxText};
use ddlog_diagnostics::{FileId, Interner, Span};
use std::borrow::Cow;

/// The main trait to go from untyped [`SyntaxNode`] to a typed ast. The
/// conversion itself has zero runtime cost since ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    fn can_cast_from(kind: SyntaxKind) -> bool;

    fn cast(syntax: &SyntaxNode) -> Option<Cow<'_, Self>>
    where
        Self: Clone;

    fn syntax(&self) -> &SyntaxNode;

    #[inline]
    fn text<'node, 'intern>(
        &'node self,
        resolver: &'intern Interner,
    ) -> SyntaxText<'node, 'intern> {
        self.syntax().resolve_text(resolver)
    }

    // FIXME: I don't like having to give the `FileId` here
    #[inline]
    fn span(&self, file: FileId) -> Span {
        let range = self.syntax().text_range();
        Span::new(range.start().into(), range.end().into(), file)
    }
}
