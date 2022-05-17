use crate::NodeCache;
use ddlog_diagnostics::{FileId, Interner};
use ddlog_utils::Arc;
use parking_lot::Mutex;

/// The maximum number of node caches we'll keep around,
/// after we have this many caches cached we'll discard any more
/// caches returned to us
const NODE_CACHE_LIMIT: usize = 16;

#[derive(Debug, Clone)]
pub struct ParserCache {
    interner: Interner,
    node_caches: Arc<Mutex<Vec<NodeCache<'static>>>>,
}

impl ParserCache {
    #[inline]
    pub fn new(interner: Interner) -> Self {
        Self {
            interner: interner.clone(),
            node_caches: Arc::new(Mutex::new(vec![NodeCache::from_interner(interner)])),
        }
    }

    #[inline]
    pub const fn interner(&self) -> &Interner {
        &self.interner
    }

    #[inline]
    pub fn file_id<P>(&self, path: P) -> FileId
    where
        P: AsRef<str>,
    {
        FileId::new(self.interner.get_or_intern(path.as_ref()))
    }

    #[inline]
    pub fn node_cache(&self) -> NodeCache<'static> {
        let mut caches = self.node_caches.lock();
        let cache = caches
            .pop()
            .unwrap_or_else(|| NodeCache::from_interner(self.interner.clone()));

        tracing::trace!(
            "popped cache from node caches, session now contains {} caches",
            caches.len(),
        );

        cache
    }

    #[inline]
    pub fn put_node_cache(&self, cache: NodeCache<'static>) {
        let mut caches = self.node_caches.lock();

        // Only push the cache when we have under our node cache limit
        if caches.len() < NODE_CACHE_LIMIT {
            caches.push(cache);

            tracing::trace!(
                "returned cache to node caches, session now contains {} caches",
                caches.len(),
            );
        }
    }
}
