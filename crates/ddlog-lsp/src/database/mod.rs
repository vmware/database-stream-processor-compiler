mod queries;

pub use queries::{Diagnostics, DocumentSymbols, Session, Source, Symbols, Validation};

use queries::{
    DiagnosticsDatabase, DocumentSymbolsDatabase, SessionDatabase, SourceDatabase, SymbolsDatabase,
    ValidationDatabase,
};
use salsa::{Database, ParallelDatabase, Snapshot, Storage};
use std::fmt::{self, Debug};

#[salsa::database(
    SourceDatabase,
    SessionDatabase,
    SymbolsDatabase,
    ValidationDatabase,
    DiagnosticsDatabase,
    DocumentSymbolsDatabase
)]
#[derive(Default)]
pub struct DDlogDatabase {
    storage: Storage<Self>,
}

impl Debug for DDlogDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DDlogDatabase").finish()
    }
}

impl Database for DDlogDatabase {}

impl ParallelDatabase for DDlogDatabase {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(Self {
            storage: self.storage.snapshot(),
        })
    }
}
