pub use crate::parser_cache::ParserCache;

use crate::{
    ast::{nodes::Root, AstNode},
    validation, RuleCtx, SyntaxNodeExt,
};
use ddlog_diagnostics::{Diagnostic, FileId, Rope};
use ddlog_utils::ArcSlice;

#[salsa::query_group(ParserCacheDatabaseStorage)]
pub trait ParserCacheDatabase {
    #[salsa::input]
    fn parser_cache(&self) -> ParserCache;

    #[salsa::transparent]
    fn file_id(&self, url: &str) -> FileId;
}

fn file_id(cache: &dyn ParserCacheDatabase, path: &str) -> FileId {
    cache.parser_cache().file_id(path)
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: ParserCacheDatabase {
    #[salsa::input]
    fn file_source(&self, file: FileId) -> Rope;

    fn parsed(&self, file: FileId) -> (Root, ArcSlice<Diagnostic>);

    fn syntax_root(&self, file: FileId) -> Root;

    fn parsing_diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;
}

fn parsed(source: &dyn SourceDatabase, file: FileId) -> (Root, ArcSlice<Diagnostic>) {
    let source_text = source.file_source(file);
    let session = source.parser_cache();

    let (parsed, cache) = crate::parse(file, &source_text.to_string(), session.node_cache());
    session.put_node_cache(cache);

    let (root, diagnostics) = parsed.into_parts();

    (root.to::<Root>().into_owned(), ArcSlice::new(diagnostics))
}

fn syntax_root(source: &dyn SourceDatabase, file: FileId) -> Root {
    source.parsed(file).0
}

fn parsing_diagnostics(source: &dyn SourceDatabase, file: FileId) -> ArcSlice<Diagnostic> {
    source.parsed(file).1
}

#[salsa::query_group(ValidationDatabaseStorage)]
pub trait ValidationDatabase: SourceDatabase {
    fn validation_diagnostics(&self, file: FileId) -> ArcSlice<Diagnostic>;
}

fn validation_diagnostics(
    validation: &dyn ValidationDatabase,
    file: FileId,
) -> ArcSlice<Diagnostic> {
    let mut ctx = RuleCtx::new(
        file,
        validation.file_source(file),
        validation.parser_cache().interner().clone(),
    );

    let root = validation.syntax_root(file);
    validation::run_default_validators(root.syntax(), &mut ctx);

    ArcSlice::new(ctx.into_diagnostics())
}
