use ddlog_diagnostics::{FileId, Interner};
use ddlog_syntax::NodeCache;
use lspower::lsp::Url;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Session {
    interner: Interner,
    node_caches: Mutex<Vec<NodeCache<'static>>>,
}

impl Session {
    pub fn new(interner: Interner) -> Self {
        Self {
            interner: interner.clone(),
            node_caches: Mutex::new(vec![NodeCache::from_interner(interner)]),
        }
    }

    pub fn node_cache(&self) -> NodeCache<'static> {
        let mut caches = self
            .node_caches
            .lock()
            .expect("a thread panicked while holding a session");

        caches
            .pop()
            .unwrap_or_else(|| NodeCache::from_interner(self.interner().clone()))
    }

    pub fn give_node_cache(&self, cache: NodeCache<'static>) {
        let mut caches = self
            .node_caches
            .lock()
            .expect("a thread panicked while holding a session");

        caches.push(cache);
    }

    pub fn interner(&self) -> &Interner {
        &self.interner
    }

    pub fn file_id(&self, url: &Url) -> FileId {
        FileId::new(self.interner().get_or_intern(url.as_str()))
    }

    /*
    // Invariant: The only way to gain access to a `FileId` is via
    // file creation or calling `.file_id()` with a valid (read:
    // previously created) url. This means we can safely add unwraps
    // and such to methods requiring a `FileId` as input, since they
    // should only refer to currently valid files.
    pub fn create_file<C>(&self, url: &Url, contents: C) -> FileId
    where
        C: Into<Rope>,
    {
        let file_id = FileId::new(self.interner().get_or_intern(url.as_str()));
        self.files.insert(file_id, contents.into());

        file_id
    }

    pub fn close_file(&self, file: FileId) {
        self.files.remove(&file);
    }

    pub fn file_id(&self, url: &Url) -> Result<FileId> {
        let file_id = FileId::new(self.interner().get_or_intern(url.as_str()));

        if self.files.contains_key(&file_id) {
            Ok(file_id)
        } else {
            Err(anyhow::format_err!("no file found for document {}", url))
        }
    }

    #[track_caller]
    pub fn file_text(&self, file: FileId) -> Rope {
        self.files
            .get(&file)
            .expect("got test for a file that doesn't exist")
            .clone()
    }
    */
}
