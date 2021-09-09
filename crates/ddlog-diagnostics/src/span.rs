use crate::FileId;
use std::{
    convert::TryFrom,
    fmt::{self, Debug, Display},
    ops::{Index, Range},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: u32,
    end: u32,
    file: FileId,
}

impl Span {
    /// Create a new span
    #[inline]
    pub const fn new(start: u32, end: u32, file: FileId) -> Self {
        Self { start, end, file }
    }

    /// Create a new span with the same start and end value
    #[inline]
    pub const fn single(span: u32, file: FileId) -> Self {
        Self::new(span, span, file)
    }

    /// Get the span's start
    #[inline]
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Get the span's start as a `Span`
    #[inline]
    pub const fn start_span(self) -> Self {
        Self::single(self.start(), self.file)
    }

    /// Get the span's end
    #[inline]
    pub const fn end(self) -> u32 {
        self.end
    }

    /// Get the span's end as a `Span`
    #[inline]
    pub const fn end_span(self) -> Self {
        Self::single(self.end(), self.file)
    }

    /// Get the span's file
    #[inline]
    pub const fn file(self) -> FileId {
        self.file
    }

    #[inline]
    pub fn merge(self, other: Self) -> Self {
        assert_eq!(
            self.file, other.file,
            "cannot merge spans from different files"
        );

        Self::new(
            self.start.min(other.start),
            self.end.max(other.end),
            self.file,
        )
    }

    #[inline]
    pub fn from_range(range: Range<usize>, file: FileId) -> Self {
        debug_assert_eq!(
            u32::try_from(range.start),
            Ok(range.start as u32),
            "range {} out of 32bit bounds (max is {})",
            range.start,
            u32::MAX,
        );
        debug_assert_eq!(
            u32::try_from(range.end),
            Ok(range.end as u32),
            "range {} out of 32bit bounds (max is {})",
            range.end,
            u32::MAX,
        );

        Self::new(range.start as u32, range.end as u32, file)
    }
}

impl ariadne::Span for Span {
    type SourceId = FileId;

    #[inline]
    fn source(&self) -> &Self::SourceId {
        &self.file
    }

    #[inline]
    fn start(&self) -> usize {
        self.start as usize
    }

    #[inline]
    fn end(&self) -> usize {
        self.end as usize
    }
}

impl Index<Span> for str {
    type Output = str;

    #[inline]
    fn index(&self, index: Span) -> &Self::Output {
        let range: Range<usize> = index.into();
        &self[range]
    }
}

impl From<Span> for Range<u32> {
    #[inline]
    fn from(range: Span) -> Self {
        range.start()..range.end()
    }
}

impl From<Span> for Range<usize> {
    #[inline]
    fn from(range: Span) -> Self {
        range.start() as usize..range.end() as usize
    }
}

impl Debug for Span {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.start, f)?;
        f.write_str("..")?;
        Debug::fmt(&self.end, f)
    }
}

impl Display for Span {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.start, f)?;
        f.write_str("..")?;
        Display::fmt(&self.end, f)
    }
}
