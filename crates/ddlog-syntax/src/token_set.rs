use crate::SyntaxKind;
use std::fmt::{self, Debug, Display};

#[allow(dead_code)]
const TOKENS_FIT_IN_SET: () =
    assert!(u128::BITS as usize * 2 > SyntaxKind::MAXIMUM_DISCRIMINANT as usize);

/// A set of [`SyntaxKind`]s
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TokenSet([u128; 2]);

impl TokenSet {
    /// Create a new token set with the given [tokens][`SyntaxKind`]
    pub const fn new(tokens: &[SyntaxKind]) -> Self {
        let (mut set, mut idx) = (Self::empty(), 0);
        while idx < tokens.len() {
            set = set.add(tokens[idx]);
            idx += 1;
        }

        set
    }

    /// Create a new token set with a single [token][`SyntaxKind`]
    pub const fn singleton(kind: SyntaxKind) -> Self {
        Self::empty().add(kind)
    }

    /// Create an empty token set
    pub const fn empty() -> Self {
        Self([0, 0])
    }

    const fn raw(self) -> [u128; 2] {
        self.0
    }

    pub const fn add(self, kind: SyntaxKind) -> Self {
        let [mut high, mut low] = self.raw();
        let mask = mask(kind);

        if kind as u16 >= 128 {
            low |= mask;
        } else {
            high |= mask;
        }

        Self([high, low])
    }

    /// Combine two token sets together
    pub const fn union(self, other: Self) -> Self {
        let [high1, low1] = self.raw();
        let [high2, low2] = other.raw();

        Self([high1 | high2, low1 | low2])
    }

    /// Returns `true` if the current token set contains the given [`SyntaxKind`]
    pub const fn contains(self, kind: SyntaxKind) -> bool {
        let [high, low] = self.raw();
        let mask = mask(kind);

        if kind as u16 >= 128 {
            low & mask != 0
        } else {
            high & mask != 0
        }
    }
}

const fn mask(kind: SyntaxKind) -> u128 {
    1u128 << (kind as usize % 128)
}

impl Debug for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set()
            .entries(
                (0..SyntaxKind::MAXIMUM_DISCRIMINANT)
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
                (0..SyntaxKind::MAXIMUM_DISCRIMINANT)
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
