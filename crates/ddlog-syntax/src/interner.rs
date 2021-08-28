use crate::hasher::ConsistentHasher;
use lasso::{Capacity, LassoResult, Spur, ThreadedRodeo};
use std::{mem::size_of, num::NonZeroUsize};
use triomphe::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Interner(Arc<ThreadedRodeo<Spur, ConsistentHasher>>);

impl Interner {
    /// Creates a new, pre-allocated [`Interner`]
    #[inline]
    pub fn new() -> Self {
        // Safety: 4096 isn't zero
        const PAGE_SIZE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(4096) };
        const STRINGS: usize = PAGE_SIZE.get() / size_of::<&str>();

        // We use `4096` as the default size and from there we find the number
        // of `&str`s that will fit within a single page and set that as the
        // initial capacity of `strings`. Then we allocate a single page for `bytes`.
        // This starts us off with a relatively high memory usage (8 kilobytes, to
        // be exact) but should save us a decent amount of allocation in the long run
        let capacity = Capacity::new(STRINGS, PAGE_SIZE);
        let interner =
            ThreadedRodeo::with_capacity_and_hasher(capacity, ConsistentHasher::default());

        Self(Arc::new(interner))
    }

    /// Creates a new, empty [`Interner`]
    ///
    /// Using [`Interner::new()`] is recommended for most cases, as it will be more
    /// efficient in the long term
    #[inline]
    pub fn empty() -> Self {
        let interner = ThreadedRodeo::with_capacity_and_hasher(
            Capacity::minimal(),
            ConsistentHasher::default(),
        );

        Self(Arc::new(interner))
    }

    #[inline]
    #[track_caller]
    pub fn get_or_intern(&self, val: &str) -> Spur {
        self.0.get_or_intern(val)
    }

    #[inline]
    pub fn try_get_or_intern(&self, val: &str) -> LassoResult<Spur> {
        self.0.try_get_or_intern(val)
    }

    #[inline]
    #[track_caller]
    pub fn get_or_intern_static(&self, val: &'static str) -> Spur {
        self.0.get_or_intern_static(val)
    }

    #[inline]
    pub fn try_get_or_intern_static(&self, val: &'static str) -> LassoResult<Spur> {
        self.0.try_get_or_intern_static(val)
    }
}

impl Default for Interner {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl lasso::Interner<Spur> for Interner {
    #[inline]
    #[track_caller]
    fn get_or_intern(&mut self, val: &str) -> Spur {
        self.0.get_or_intern(val)
    }

    #[inline]
    fn try_get_or_intern(&mut self, val: &str) -> LassoResult<Spur> {
        self.0.try_get_or_intern(val)
    }

    #[inline]
    #[track_caller]
    fn get_or_intern_static(&mut self, val: &'static str) -> Spur {
        self.0.get_or_intern_static(val)
    }

    #[inline]
    fn try_get_or_intern_static(&mut self, val: &'static str) -> LassoResult<Spur> {
        self.0.try_get_or_intern_static(val)
    }
}

impl lasso::Reader<Spur> for Interner {
    #[inline]
    fn get(&self, val: &str) -> Option<Spur> {
        self.0.get(val)
    }

    #[inline]
    fn contains(&self, val: &str) -> bool {
        self.0.contains(val)
    }
}

impl lasso::Resolver<Spur> for Interner {
    #[inline]
    #[track_caller]
    fn resolve<'a>(&'a self, key: &Spur) -> &'a str {
        self.0.resolve(key)
    }

    #[inline]
    fn try_resolve<'a>(&'a self, key: &Spur) -> Option<&'a str> {
        self.0.try_resolve(key)
    }

    #[inline]
    unsafe fn resolve_unchecked<'a>(&'a self, key: &Spur) -> &'a str {
        self.0.resolve_unchecked(key)
    }

    #[inline]
    fn contains_key(&self, key: &Spur) -> bool {
        self.0.contains_key(key)
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}
