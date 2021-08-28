use std::{
    cmp::Ordering,
    fmt::{self, Debug},
    hash::BuildHasher,
};
use xxhash_rust::{const_xxh3::const_custom_default_secret, xxh3::Xxh3};

/// A consistent and fast hasher
pub struct ConsistentHasher(());

impl ConsistentHasher {
    /// Create a new consistent hasher
    #[inline]
    pub const fn new() -> Self {
        Self(())
    }

    /// The seed used to derive the inner xxh3 hasher's secret
    #[allow(clippy::unusual_byte_groupings)]
    const HASH_SEED: u64 = 0xBAD_A55_BEEF;

    /// The inner xxh3 hasher's secret
    const HASH_SECRET: [u8; 192] = const_custom_default_secret(Self::HASH_SEED);

    /// The hasher to be used for every [`ConsistentHasher`] instance
    const DEFAULT_HASHER: Xxh3 = Xxh3::with_secret(Self::HASH_SECRET);
}

impl Default for ConsistentHasher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for ConsistentHasher {
    type Hasher = Xxh3;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        Self::DEFAULT_HASHER
    }
}

impl Clone for ConsistentHasher {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Copy for ConsistentHasher {}

impl Ord for ConsistentHasher {
    #[inline]
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for ConsistentHasher {
    #[inline]
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Eq for ConsistentHasher {}

impl PartialEq for ConsistentHasher {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Debug for ConsistentHasher {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConsistentHasher").finish()
    }
}
