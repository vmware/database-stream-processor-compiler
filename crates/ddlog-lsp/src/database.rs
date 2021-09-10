use std::fmt::{self, Debug};

use ddlog_diagnostics::{Diagnostic, FileId, Rope};
use ddlog_syntax::{validation, RuleCtx, SyntaxNode};
use lspower::lsp::Url;
use salsa::{Database, ParallelDatabase, Snapshot, Storage};
use triomphe::Arc;

#[salsa::query_group(SessionDatabase)]
pub trait Session {
    #[salsa::input]
    fn session(&self) -> Arc<crate::Session>;

    #[salsa::transparent]
    fn file_id(&self, url: &Url) -> FileId;
}

fn file_id(session: &dyn Session, url: &Url) -> FileId {
    FileId::new(session.session().interner().get_or_intern(url.as_str()))
}

#[salsa::query_group(SourceDatabase)]
pub trait Source: Session {
    #[salsa::input]
    fn file_source(&self, file: FileId) -> Rope;

    fn parsed(&self, file: FileId) -> (SyntaxNode, Arc<Vec<Diagnostic>>);

    fn parse_diagnostics(&self, file: FileId) -> Arc<Vec<Diagnostic>>;

    fn syntax(&self, file: FileId) -> SyntaxNode;
}

fn parsed(source: &dyn Source, file: FileId) -> (SyntaxNode, Arc<Vec<Diagnostic>>) {
    let source_text = source.file_source(file);
    let session = source.session();

    let (parsed, cache) = ddlog_syntax::parse(file, &source_text.to_string(), session.node_cache());
    session.give_node_cache(cache);

    let (root, mut diagnostics) = parsed.into_parts();
    diagnostics.shrink_to_fit();

    (root, Arc::new(diagnostics))
}

fn syntax(source: &dyn Source, file: FileId) -> SyntaxNode {
    source.parsed(file).0
}

fn parse_diagnostics(source: &dyn Source, file: FileId) -> Arc<Vec<Diagnostic>> {
    source.parsed(file).1
}

#[salsa::query_group(ValidationDatabase)]
pub trait Validation: Source {
    fn validation_diagnostics(&self, file: FileId) -> Arc<Vec<Diagnostic>>;
}

fn validation_diagnostics(validation: &dyn Validation, file: FileId) -> Arc<Vec<Diagnostic>> {
    let mut ctx = RuleCtx::new(
        file,
        validation.file_source(file),
        validation.session().interner().clone(),
    );

    validation::run_validators(&validation.syntax(file), &mut ctx);
    ctx.diagnostics.shrink_to_fit();

    Arc::new(ctx.diagnostics)
}

#[salsa::database(SessionDatabase, SourceDatabase, ValidationDatabase)]
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
