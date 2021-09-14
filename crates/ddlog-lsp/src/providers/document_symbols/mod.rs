mod database;

pub(crate) use database::{
    declarations, document_function, document_function_arg, document_relation,
    document_relation_column, document_symbols,
};

use crate::database::{DDlogDatabase, Session, Symbols};
use lspower::lsp::{DocumentSymbolResponse, Url};
use salsa::Snapshot;

pub(crate) fn nested_symbols(
    snapshot: Snapshot<DDlogDatabase>,
    url: &Url,
) -> Option<DocumentSymbolResponse> {
    let session = snapshot.session();
    let file = session.file_id(url);

    let symbols = snapshot.document_symbols(file);
    if symbols.is_empty() {
        tracing::trace!(
            file = %url,
            "didn't generate any symbols, returning `None` to the client",
        );

        None
    } else {
        Some(DocumentSymbolResponse::Nested(symbols.to_vec()))
    }
}
