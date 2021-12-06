#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Returns `true` if the `Either` is [`Left`].
    ///
    /// [`Left`]: Either::Left
    pub const fn is_left(&self) -> bool {
        matches!(self, Self::Left(..))
    }

    /// Returns `true` if the `Either` is [`Right`].
    ///
    /// [`Right`]: Either::Right
    pub const fn is_right(&self) -> bool {
        matches!(self, Self::Right(..))
    }

    pub const fn as_left(&self) -> Option<&L> {
        if let Self::Left(left) = self {
            Some(left)
        } else {
            None
        }
    }

    pub const fn as_right(&self) -> Option<&R> {
        if let Self::Right(right) = self {
            Some(right)
        } else {
            None
        }
    }

    pub fn try_into_left(self) -> Result<L, Self> {
        if let Self::Left(left) = self {
            Ok(left)
        } else {
            Err(self)
        }
    }

    pub fn try_into_right(self) -> Result<R, Self> {
        if let Self::Right(right) = self {
            Ok(right)
        } else {
            Err(self)
        }
    }
}
