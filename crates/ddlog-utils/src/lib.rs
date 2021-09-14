//! Various utilities shared between various ddlog crates

mod arc_slice;
mod hasher;

pub use arc_slice::ArcSlice;
pub use hasher::ConsistentHasher;
pub use triomphe::Arc;
