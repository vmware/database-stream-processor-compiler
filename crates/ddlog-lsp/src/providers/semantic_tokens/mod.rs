//! Semantic Tokens helpers

mod builder;
mod highlighter;
pub(crate) mod tokens;

use crate::{providers::semantic_tokens::highlighter::SemanticHighlighter, Session};
use anyhow::Result;
use lspower::lsp::{SemanticTokens, SemanticTokensParams};

pub(crate) fn handle_semantic_tokens_full(
    session: &Session,
    params: SemanticTokensParams,
) -> Result<SemanticTokens> {
    let file = session.file_id(&params.text_document.uri)?;
    let text = session.file_text(file);

    // FIXME: Allow the parser & lexer to directly operate off of ropes
    let (parsed, cache) = ddlog_syntax::parse(file, &text.to_string(), session.node_cache());
    session.give_node_cache(cache);

    // TODO: Token caching
    // TODO: Cache syntax trees
    let semantic_tokens = SemanticHighlighter::highlight(&text, parsed.root());

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
