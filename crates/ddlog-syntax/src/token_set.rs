use crate::SyntaxKind;
use std::fmt::{self, Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TokenSet(u128);

impl TokenSet {
    /// Create a new token set with the given [tokens][`SyntaxKind`]
    pub const fn new(tokens: &[SyntaxKind]) -> Self {
        let (mut set, mut idx) = (0, 0);
        while idx < tokens.len() {
            set |= mask(tokens[idx]);
            idx += 1;
        }

        Self(set)
    }

    /// Create a new token set with a single [token][`SyntaxKind`]
    pub const fn singleton(kind: SyntaxKind) -> Self {
        Self(mask(kind))
    }

    /// Create an empty token set
    pub const fn empty() -> Self {
        Self(0)
    }

    const fn raw(self) -> u128 {
        self.0
    }

    pub const fn add(self, kind: SyntaxKind) -> Self {
        Self(self.raw() | mask(kind))
    }

    /// Combine two token sets together
    pub const fn union(self, other: Self) -> Self {
        Self(self.raw() | other.raw())
    }

    /// Returns `true` if the current token set contains the given [`SyntaxKind`]
    pub const fn contains(self, kind: SyntaxKind) -> bool {
        self.raw() & mask(kind) != 0
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize)
}

impl Debug for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(
                (0..SyntaxKind::highest() as u16)
                    .map(SyntaxKind::from)
                    .filter(|&kind| self.contains(kind)),
            )
            .finish()
    }
}

impl Display for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DisplayKind(SyntaxKind);

        impl Debug for DisplayKind {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                Display::fmt(&self.0, f)
            }
        }

        f.debug_set()
            .entries(
                (0..SyntaxKind::highest() as u16)
                    .map(SyntaxKind::from)
                    .filter(|&kind| self.contains(kind))
                    .map(DisplayKind),
            )
            .finish()
    }
}

/// Utility macro for making a new token set
#[macro_export]
macro_rules! token_set {
    () => { $crate::TokenSet::empty() };

    ($($token:expr),* $(,)?) => {
        $crate::TokenSet::new(&[
            $($token,)*
        ])
    };

    ($($token:expr),* $(,)?) => {
        $crate::token_set!($($token),*)
    };
}
