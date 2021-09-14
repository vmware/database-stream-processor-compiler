use ddlog_diagnostics::{FileId, Interner};
use ddlog_syntax::NodeCache;
use lspower::lsp::Url;
// TODO: Probably want to switch to use `parking_lot`
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

        let cache = caches
            .pop()
            .unwrap_or_else(|| NodeCache::from_interner(self.interner().clone()));

        tracing::trace!(
            "session contains {} node caches (just took one)",
            caches.len(),
        );

        cache
    }

    pub fn give_node_cache(&self, cache: NodeCache<'static>) {
        let mut caches = self
            .node_caches
            .lock()
            .expect("a thread panicked while holding a session");

        caches.push(cache);
        tracing::trace!(
            "session contains {} node caches (just given one)",
            caches.len(),
        );
    }

    pub fn interner(&self) -> &Interner {
        &self.interner
    }

    pub fn file_id(&self, url: &Url) -> FileId {
        FileId::new(self.interner().get_or_intern(url.as_str()))
    }
}
