use crate::{IStr, Interner};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FileId(IStr);

impl FileId {
    #[inline]
    pub const fn new(path: IStr) -> Self {
        Self(path)
    }

    #[inline]
    pub fn to_str(self, interner: &Interner) -> &str {
        interner.resolve(self.0)
    }
}
