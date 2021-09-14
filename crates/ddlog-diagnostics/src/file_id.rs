use crate::{IStr, Interner};
use std::fmt::{self, Debug, Write};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    const fn into_inner(self) -> IStr {
        self.0
    }
}

impl Debug for FileId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FileId(")?;
        Debug::fmt(&self.into_inner().into_inner().into_inner(), f)?;
        f.write_char(')')
    }
}
