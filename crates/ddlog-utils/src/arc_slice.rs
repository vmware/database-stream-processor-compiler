use std::{
    fmt::{self, Debug},
    ops::Deref,
};
use triomphe::ThinArc;

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArcSlice<T> {
    slice: ThinArc<(), T>,
}

impl<T> ArcSlice<T> {
    pub fn new<I>(slice: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        Self {
            slice: ThinArc::from_header_and_iter((), slice.into_iter()),
        }
    }

    /// Creates a new [`ArcSlice`] from an iterator of unknown size. Whenever possible
    /// [`ArcSlice::new()`] should be used as it's much more efficient.
    pub fn from_dynamic<I>(slice: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        // We need to collect into a `Vec` in order to create an exactly sized iterator
        Self::new(slice.into_iter().collect::<Vec<_>>())
    }
}

impl<T> Deref for ArcSlice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.slice.slice
    }
}

impl<T> Clone for ArcSlice<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            slice: self.slice.clone(),
        }
    }
}

impl<T> Debug for ArcSlice<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.slice.slice).finish()
    }
}
