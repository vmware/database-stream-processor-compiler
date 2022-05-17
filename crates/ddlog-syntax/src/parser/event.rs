use crate::{Span, SyntaxKind};
use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Enter {
        kind: SyntaxKind,
        preceded_by: Option<NonZeroUsize>,
    },
    Exit,
    Token {
        kind: SyntaxKind,
        span: Span,
    },
}

impl Event {
    #[inline]
    pub const fn tombstone() -> Self {
        Self::Enter {
            kind: SyntaxKind::TOMBSTONE,
            preceded_by: None,
        }
    }

    #[inline]
    pub const fn is_tombstone(self) -> bool {
        matches!(
            self,
            Self::Enter {
                kind: SyntaxKind::TOMBSTONE,
                preceded_by: None,
            },
        )
    }
}

impl Default for Event {
    #[inline]
    fn default() -> Self {
        Self::tombstone()
    }
}
