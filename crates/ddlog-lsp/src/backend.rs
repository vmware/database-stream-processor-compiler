use crate::{
    database::{DDlogDatabase, Session as _, Source, Validation},
    providers::{
        self,
        semantic_tokens::tokens::{SUPPORTED_MODIFIERS, SUPPORTED_TYPES},
        utils,
    },
    Session,
};
use cstree::TextRange;
use ddlog_diagnostics::{Diagnostic, FileId, Level, Rope};
use lsp_text::RopeExt;
use lspower::{
    jsonrpc::Result,
    lsp::{
        Diagnostic as LspDiagnostic, DiagnosticRelatedInformation, DiagnosticSeverity,
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
        Location, MessageType, NumberOrString, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
        SemanticTokensServerCapabilities, ServerCapabilities, TextDocumentSyncCapability,
        TextDocumentSyncKind, TextDocumentSyncOptions, Url,
    },
    Client, LanguageServer,
};
use salsa::{ParallelDatabase, Snapshot};
use std::{env, ffi::OsStr, fmt::Display, fs, path::PathBuf, str::FromStr, sync::Mutex};
use tokio::task;
use triomphe::Arc;
use walkdir::WalkDir;

const DDLOG_LANG: &str = "ddlog";
const DDLOG_DAT_LANG: &str = "ddlog-command";

#[derive(Debug)]
pub struct Backend {
    client: Client,
    session: Arc<Session>,
    database: Arc<Mutex<DDlogDatabase>>,
}

impl Backend {
    pub fn new(client: Client, session: Arc<Session>) -> Self {
        Self {
            client,
            database: Arc::new(Mutex::new(DDlogDatabase::default())),
            session,
        }
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

    pub async fn publish_diagnostics(
        &self,
        file: FileId,
        diagnostics: Vec<Diagnostic>,
        snapshot: Snapshot<DDlogDatabase>,
    ) {
        self.publish_diagnostics_for(file, diagnostics, None, snapshot)
            .await;
    }

    pub async fn publish_diagnostics_for(
        &self,
        file: FileId,
        diagnostics: Vec<Diagnostic>,
        version: Option<i32>,
        snapshot: Snapshot<DDlogDatabase>,
    ) {
        let uri = Url::from_str(file.to_str(self.session.interner())).unwrap();
        let source = snapshot.file_source(file);

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

        self.database
            .lock()
            .unwrap()
            .set_session(self.session.clone());

        if let Ok(ddlog_home) = env::var("DDLOG_HOME") {
            let ddlog_home = PathBuf::from(ddlog_home);

            if ddlog_home.exists() {
                task::spawn_blocking(move || {
                    let ddlog_libs = WalkDir::new(&ddlog_home)
                        .into_iter()
                        .flatten()
                        // Filter out anything that's not a `.dl` file
                        .filter(|entry| {
                            entry.file_type().is_file()
                                && matches!(
                                    entry.path().extension().and_then(OsStr::to_str),
                                    Some("dl"),
                                )
                        });

                    for file in ddlog_libs {
                        let contents = fs::read_to_string(file.path()).unwrap();
                        println!("{}", contents);

                        // TODO: Load stdlib
                    }
                });
            } else {
                self.error(format!(
                    "DDLOG_HOME dir does not exist: '{}'",
                    ddlog_home.display(),
                ))
                .await;
            }
        }

        Ok(InitializeResult {
            server_info: None,
            capabilities,
        })
    }

    async fn shutdown(&self) -> Result<()> {
        self.info("shutting down").await;

        Ok(())
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
            let file = self.session.file_id(&opened.text_document.uri);

            let snapshot = {
                let mut database = self.database.lock().unwrap();
                database.set_file_source(file, Rope::from(opened.text_document.text));

                database.snapshot()
            };

            let mut diagnostics = (*snapshot.parse_diagnostics(file)).clone();
            diagnostics.extend((*snapshot.validation_diagnostics(file)).clone());

            if !diagnostics.is_empty() {
                self.publish_diagnostics_for(
                    file,
                    diagnostics,
                    Some(opened.text_document.version),
                    snapshot,
                )
                .await;
            }
        }
    }

    async fn did_change(&self, changes: DidChangeTextDocumentParams) {
        let file_name = changes.text_document.uri.as_str();

        let file = self.session.file_id(&changes.text_document.uri);
        self.info(format!("file changed: {}", file_name)).await;

        let snapshot = {
            let mut database = self.database.lock().unwrap();
            let mut contents = database.file_source(file);

            for change in changes.content_changes {
                // TODO: Error handling
                let edit = contents
                    .build_edit(&change)
                    .expect("failed to create text edit");

                contents.apply_edit(&edit);
            }

            database.set_file_source(file, contents);

            database.snapshot()
        };

        let mut diagnostics = (*snapshot.parse_diagnostics(file)).clone();
        diagnostics.extend((*snapshot.validation_diagnostics(file)).clone());

        if !diagnostics.is_empty() {
            self.publish_diagnostics_for(
                file,
                diagnostics,
                Some(changes.text_document.version),
                snapshot,
            )
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
            providers::semantic_tokens::handle_semantic_tokens_full(
                self.database.lock().unwrap().snapshot(),
                file_name,
            )
            .unwrap(),
        )))
    }
}
