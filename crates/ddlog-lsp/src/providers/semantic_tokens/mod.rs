//! Semantic Tokens helpers

mod builder;
mod highlighter;
pub(crate) mod tokens;

use crate::{
    database::{DDlogDatabase, Session, Source},
    providers::semantic_tokens::highlighter::SemanticHighlighter,
};
use anyhow::Result;
use ddlog_syntax::{ast::nodes::Root, SyntaxNodeExt};
use lspower::lsp::{SemanticTokens, Url};
use salsa::Snapshot;

pub(crate) fn handle_semantic_tokens_full(
    snapshot: Snapshot<DDlogDatabase>,
    url: &Url,
) -> Result<SemanticTokens> {
    let session = snapshot.session();
    let file = session.file_id(url);

    let text = snapshot.file_source(file);
    let root = snapshot.syntax(file);

    let semantic_tokens =
        SemanticHighlighter::highlight(&text, &*root.to::<Root>(), session.interner());

    Ok(semantic_tokens)
}

/*
pub(crate) fn handle_semantic_tokens_full_delta(
    params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    let file_id = from_proto::file_id(&snap, &params.text_document.uri)?;
    let text = snap.analysis.file_text(file_id)?;
    let line_index = snap.file_line_index(file_id)?;

    let highlights = snap.analysis.highlight(file_id)?;
    let highlight_strings = snap.config.highlighting_strings();
    let semantic_tokens =
        to_proto::semantic_tokens(&text, &line_index, highlights, highlight_strings);

    let mut cache = snap.semantic_tokens_cache.lock();
    let cached_tokens = cache.entry(params.text_document.uri).or_default();

    if let Some(prev_id) = &cached_tokens.result_id {
        if *prev_id == params.previous_result_id {
            let delta = to_proto::semantic_token_delta(cached_tokens, &semantic_tokens);
            *cached_tokens = semantic_tokens;
            return Ok(Some(delta.into()));
        }
    }

    *cached_tokens = semantic_tokens.clone();

    Ok(Some(semantic_tokens.into()))
}

pub(crate) fn handle_semantic_tokens_range(
    params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    let frange = from_proto::file_range(&snap, params.text_document, params.range)?;
    let text = snap.analysis.file_text(frange.file_id)?;
    let line_index = snap.file_line_index(frange.file_id)?;

    let highlights = snap.analysis.highlight_range(frange)?;
    let highlight_strings = snap.config.highlighting_strings();
    let semantic_tokens =
        to_proto::semantic_tokens(&text, &line_index, highlights, highlight_strings);

    Ok(Some(semantic_tokens.into()))
}
*/
