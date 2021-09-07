use crate::{
    providers::{
        self,
        semantic_tokens::tokens::{SUPPORTED_MODIFIERS, SUPPORTED_TYPES},
        utils,
    },
    Session,
};
use cstree::TextRange;
use ddlog_diagnostics::{Diagnostic, FileId, Level};
use ddlog_syntax::{validation, RuleCtx};
use lsp_text::RopeExt;
use lspower::{
    jsonrpc::Result,
    lsp::{
        Diagnostic as LspDiagnostic, DiagnosticRelatedInformation, DiagnosticSeverity,
        DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
        DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams, ExecuteCommandParams, InitializeParams, InitializeResult,
        InitializedParams, Location, MessageType, NumberOrString, SemanticTokensDeltaParams,
        SemanticTokensFullDeltaResult, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensParams, SemanticTokensRangeParams,
        SemanticTokensRangeResult, SemanticTokensResult, SemanticTokensServerCapabilities,
        ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
        TextDocumentSyncOptions, Url, WorkspaceEdit,
    },
    Client, LanguageServer,
};
use serde_json::Value;
use std::{fmt::Display, str::FromStr};
use triomphe::Arc;

const DDLOG_LANG: &str = "ddlog";
const DDLOG_DAT_LANG: &str = "ddlog-command";

#[derive(Debug)]
pub struct Backend {
    client: Client,
    session: Arc<Session>,
}

impl Backend {
    pub fn new(client: Client, session: Arc<Session>) -> Self {
        Self { client, session }
    }

    pub async fn info<M>(&self, message: M)
    where
        M: Display,
    {
        self.client.log_message(MessageType::Info, message).await;
    }

    pub async fn warn<M>(&self, message: M)
    where
        M: Display,
    {
        self.client.log_message(MessageType::Warning, message).await;
    }

    pub async fn error<M>(&self, message: M)
    where
        M: Display,
    {
        self.client.log_message(MessageType::Error, message).await;
    }

    pub async fn publish_diagnostics(&self, file: FileId, diagnostics: Vec<Diagnostic>) {
        self.publish_diagnostics_for(file, diagnostics, None).await;
    }

    pub async fn publish_diagnostics_for(
        &self,
        file: FileId,
        diagnostics: Vec<Diagnostic>,
        version: Option<i32>,
    ) {
        let uri = Url::from_str(file.to_str(self.session.interner())).unwrap();
        let source = self.session.file_text(file);

        // TODO: Factor this conversion out into utility function(s)
        let diagnostics: Vec<_> = diagnostics.into_iter().map(|diagnostic| {
            let primary_span = diagnostic.message_span.or_else(|| {
                diagnostic.labels
                    .iter()
                    .find_map(|label| label.is_primary.then(|| label.span))
            })
            .expect("expected a primary label or a message span within a diagnostic but failed to get one");

            let range = utils::ide_range(&source, TextRange::new(primary_span.start().into(), primary_span.end().into()));
            let level = match diagnostic.level {
                Level::Error => DiagnosticSeverity::Error,
                Level::Warning => DiagnosticSeverity::Warning,
                Level::Note => DiagnosticSeverity::Information,
            };
            let code = diagnostic.code.map(|code| NumberOrString::Number(code as i32));
            let labels: Vec<_> = diagnostic.labels.into_iter().map(|label| {
                DiagnosticRelatedInformation {
                    message: label.message.unwrap().into_owned(),
                    location: Location {
                        uri: uri.clone(),
                        range: utils::ide_range(&source, TextRange::new(label.span.start().into(), label.span.end().into()))
                    },
                }
            }).collect();

            LspDiagnostic::new(
                range,
                Some(level),
                code,
                Some("ddlog-lsp".to_string()),
                diagnostic.message.unwrap().into_owned(),
                Some(labels),
                None,
            )
        })
        .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, version)
            .await;
    }
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.info(format!("initializing: {:?}", params)).await;

        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::Incremental),
                    ..Default::default()
                },
            )),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: SUPPORTED_TYPES.to_vec(),
                        token_modifiers: SUPPORTED_MODIFIERS.to_vec(),
                    },
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    ..Default::default()
                }),
            ),
            ..Default::default()
        };

        Ok(InitializeResult {
            server_info: None,
            capabilities,
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        self.info(format!("initialized: {:?}", params)).await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.info("shutting down").await;

        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.info("workspace folders changed!").await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.info("configuration changed!").await;
    }

    async fn did_change_watched_files(&self, files: DidChangeWatchedFilesParams) {
        self.info(format!("watched files changed: {:?}", files))
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.info("command executed!").await;

        match self
            .client
            .apply_edit(WorkspaceEdit::default(), Default::default())
            .await
        {
            Ok(res) if res.applied => self.info("applied").await,
            Ok(_) => self.info("rejected").await,
            Err(error) => self.error(error).await,
        }

        Ok(None)
    }

    async fn did_open(&self, opened: DidOpenTextDocumentParams) {
        let file_name = opened.text_document.uri.as_str();
        self.info(format!(
            "file opened: {} (lang: {})",
            file_name, opened.text_document.language_id,
        ))
        .await;

        if opened.text_document.language_id == DDLOG_LANG
            || opened.text_document.language_id == DDLOG_DAT_LANG
        {
            let file = self
                .session
                .create_file(&opened.text_document.uri, opened.text_document.text);
            let source = self.session.file_text(file);

            // FIXME: Allow the parser & lexer to directly operate off of ropes
            // FIXME: Cache syntax trees
            tracing::trace!("started parsing");
            let (parsed, cache) =
                ddlog_syntax::parse(file, &source.to_string(), self.session.node_cache());
            self.session.give_node_cache(cache);
            tracing::trace!("finished parsing: {}", parsed.debug_tree());

            let mut ctx = RuleCtx::new(file, source, self.session.interner().clone());

            validation::run_validators(parsed.syntax(), &mut ctx);
            ctx.diagnostics.extend(parsed.into_errors());

            if !ctx.diagnostics.is_empty() {
                self.publish_diagnostics_for(
                    file,
                    ctx.diagnostics,
                    Some(opened.text_document.version),
                )
                .await;
            }
        }
    }

    async fn did_change(&self, changes: DidChangeTextDocumentParams) {
        let file_name = changes.text_document.uri.as_str();

        if let Ok(file_id) = self.session.file_id(&changes.text_document.uri) {
            self.info(format!("file changed: {}", file_name)).await;

            if let Some(mut contents) = self.session.files.get_mut(&file_id) {
                for change in changes.content_changes {
                    // TODO: Error handling
                    let edit = contents
                        .build_edit(&change)
                        .expect("failed to create text edit");

                    contents.apply_edit(&edit);
                }
            } else {
                self.error(format!(
                    "tried to change file that doesn't exist: {}",
                    file_name,
                ))
                .await;
            }
        } else {
            self.warn(format!("unregistered file changed: {}", file_name))
                .await;
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.info("file saved!").await;
    }

    async fn did_close(&self, closed: DidCloseTextDocumentParams) {
        let file_name = closed.text_document.uri.as_str();

        if let Ok(file_id) = self.session.file_id(&closed.text_document.uri) {
            self.info(format!("file closed: {}", file_name)).await;
            self.session.close_file(file_id);
        } else {
            self.warn(format!("unregistered file closed: {}", file_name))
                .await;
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let file_name = &params.text_document.uri;
        self.info(format!("semantic tokens full: {}", file_name))
            .await;

        Ok(Some(SemanticTokensResult::Tokens(
            // FIXME: Error handling
            providers::semantic_tokens::handle_semantic_tokens_full(&self.session, file_name)
                .unwrap(),
        )))
    }

    async fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        let file_name = &params.text_document.uri;
        self.info(format!("semantic tokens full delta: {}", file_name))
            .await;

        Ok(Some(SemanticTokensFullDeltaResult::Tokens(
            // FIXME: Error handling
            providers::semantic_tokens::handle_semantic_tokens_full(&self.session, file_name)
                .unwrap(),
        )))
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let file_name = &params.text_document.uri;
        self.info(format!("semantic tokens range: {}", file_name))
            .await;

        Ok(Some(SemanticTokensRangeResult::Tokens(
            // FIXME: Error handling
            providers::semantic_tokens::handle_semantic_tokens_full(&self.session, file_name)
                .unwrap(),
        )))
    }
}
