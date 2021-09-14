use crate::{FileId, Interner};
use ariadne::{Cache, Source};
use ddlog_utils::ConsistentHasher;
use ropey::Rope;
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Debug, Display},
};

pub struct FileCache {
    files: HashMap<FileId, Source, ConsistentHasher>,
    interner: Interner,
}

impl FileCache {
    #[inline]
    pub fn new(interner: Interner) -> Self {
        Self {
            files: HashMap::with_hasher(ConsistentHasher::default()),
            interner,
        }
    }

    #[inline]
    pub fn add_str(&mut self, file: FileId, contents: &str) {
        let source = Source::from(contents);
        self.files.insert(file, source);
    }

    #[inline]
    pub fn add(&mut self, file: FileId, contents: Rope) {
        let contents = Cow::from(contents);
        let source = Source::from(contents);

        self.files.insert(file, source);
    }

    #[inline]
    pub fn remove(&mut self, file: FileId) {
        self.files.remove(&file);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.files.clear();
    }
}

impl Cache<FileId> for FileCache {
    #[inline]
    fn fetch(&mut self, id: &FileId) -> Result<&Source, Box<dyn Debug>> {
        self.files.get(id).ok_or_else(|| {
            Box::new(format!(
                "could not find file '{}'",
                id.to_str(&self.interner)
            )) as Box<_>
        })
    }

    #[inline]
    fn display<'a>(&self, id: &'a FileId) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(id.to_str(&self.interner).to_owned()) as Box<_>)
    }
}
