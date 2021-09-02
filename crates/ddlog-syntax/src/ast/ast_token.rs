use ddlog_diagnostics::Interner;

use crate::{SyntaxKind, SyntaxToken};

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
}
