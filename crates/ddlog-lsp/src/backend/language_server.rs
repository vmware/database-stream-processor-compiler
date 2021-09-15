use crate::{
    backend::{DDLOG_DAT_LANG, DDLOG_LANG},
    database::{Diagnostics, Session as _, Source},
    lsp_info,
    providers::{
        document_symbols,
        semantic_tokens::{
            self,
            tokens::{SUPPORTED_MODIFIERS, SUPPORTED_TYPES},
        },
    },
    Backend,
};
use ddlog_diagnostics::Rope;
use lsp_text::RopeExt;
use lspower::{
    jsonrpc::Result,
    lsp::{
        DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentSymbolParams,
        DocumentSymbolResponse, ExecuteCommandParams, InitializeParams, InitializeResult, OneOf,
        SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
        SemanticTokensParams, SemanticTokensResult, SemanticTokensServerCapabilities,
        ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
        TextDocumentSyncOptions,
    },
    LanguageServer,
};
use salsa::ParallelDatabase;
use serde_json::Value;

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        tracing::info!(?params, "received initialization request");
        lsp_info!(self, "received initialization request").await;

        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::Incremental),
                    ..Default::default()
                },
            )),
            document_symbol_provider: Some(OneOf::Left(true)),
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
        tracing::info!(?capabilities, "requesting lsp capabilities");

        self.database
            .lock()
            .expect("failed to lock database")
            .set_session(self.session.clone());

        /*
        if let Ok(ddlog_home) = env::var("DDLOG_HOME") {
            let ddlog_home = PathBuf::from(ddlog_home);

            if ddlog_home.exists() {
                let (database, interner) = (
                    self.database.clone(),
                    self.session.interner().clone(),
                    // self.client.clone(),
                );

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

                    let (snapshot, files, mut cache) = {
                        let mut database = database.lock().unwrap();
                        let mut cache = FileCache::new(interner.clone());
                        let files: Vec<_> = ddlog_libs
                            .map(|file| {
                                let uri = format!(
                                    "file:{}",
                                    file.path().canonicalize().unwrap().display(),
                                );
                                let file_id = FileId::new(interner.get_or_intern(&uri));

                                let contents = fs::read_to_string(file.path()).unwrap();
                                database.set_file_source(file_id, Rope::from(contents));

                                cache.add(file_id, database.file_source(file_id));

                                file_id
                            })
                            .collect();

                        (database.snapshot(), files, cache)
                    };

                    for file in files {
                        let mut diagnostics = (*snapshot.parse_diagnostics(file)).clone();
                        diagnostics.extend((*snapshot.validation_diagnostics(file)).clone());

                        // Can't do this since VSCode doesn't know that these files exist?
                        //     snapshot = Backend::publish_diagnostics_raw(
                        //         file,
                        //         diagnostics,
                        //         None,
                        //         snapshot,
                        //         &interner,
                        //         &client,
                        //     )
                        //     .await;

                        let config = DiagnosticConfig::default();
                        let stdout = io::stdout();
                        let mut stdout = stdout.lock();

                        for diagnostic in diagnostics {
                            diagnostic
                                .emit_to(&config, &mut cache, &mut stdout)
                                .unwrap();
                        }
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
        */

        Ok(InitializeResult {
            server_info: None,
            capabilities,
        })
    }

    async fn shutdown(&self) -> Result<()> {
        tracing::info!("received shutdown request");
        lsp_info!(self, "shutting down").await;

        Ok(())
    }

    async fn did_open(&self, opened: DidOpenTextDocumentParams) {
        let (file, language, version) = (
            &opened.text_document.uri,
            &opened.text_document.language_id,
            opened.text_document.version,
        );

        // See if the opened document's language id is one of the two that we accept
        let is_accepted_language = language == DDLOG_LANG || language == DDLOG_DAT_LANG;
        tracing::info!(
            %file,
            %language,
            version,
            is_accepted_language,
            "opened a new text document",
        );

        if is_accepted_language {
            let file = self.session.file_id(file);

            let snapshot = {
                let mut database = self.database.lock().expect("failed to lock database");
                database.set_file_source(file, Rope::from(opened.text_document.text));

                database.snapshot()
            };

            let diagnostics = snapshot.diagnostics(file).to_vec();
            self.publish_diagnostics_for(file, diagnostics, Some(version), snapshot)
                .await;
        } else {
            tracing::info!(
                %file,
                %language,
                version,
                "ignored opened text document, {:?} is not contained within [{:?}, {:?}]",
                language,
                DDLOG_LANG,
                DDLOG_DAT_LANG,
            );
        }
    }

    async fn did_change(&self, changes: DidChangeTextDocumentParams) {
        let (uri, version) = (&changes.text_document.uri, changes.text_document.version);
        let file = self.session.file_id(uri);

        tracing::info!(
            file = ?uri,
            version,
            "received file change request",
        );

        let snapshot = {
            let mut database = self.database.lock().expect("failed to lock database");
            let mut contents = database.file_source(file);

            tracing::debug!(
                file = ?uri,
                version,
                "applying {} content changes",
                changes.content_changes.len(),
            );

            for change in changes.content_changes {
                match contents.build_edit(&change) {
                    Ok(edit) => contents.apply_edit(&edit),
                    Err(error) => {
                        tracing::error!(
                            file = ?uri,
                            version,
                            ?change,
                            "failed to apply edit: {}",
                            error,
                        );
                    }
                }
            }

            database.set_file_source(file, contents);
            database.snapshot()
        };

        let diagnostics = snapshot.diagnostics(file).to_vec();
        self.publish_diagnostics_for(file, diagnostics, Some(version), snapshot)
            .await;
    }

    // TODO: We want delta and incremental results as well
    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = &params.text_document.uri;
        tracing::info!(
            file = ?uri,
            "received full semantic token request",
        );

        let snapshot = self.snapshot();
        Ok(semantic_tokens::semantic_tokens_full(snapshot, uri))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;
        tracing::info!(
            file = ?uri,
            "received document symbol request",
        );

        let snapshot = self.snapshot();
        Ok(document_symbols::nested_symbols(snapshot, uri))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        tracing::info!(?params, "received execute command request");

        Ok(None)
    }

    // These requests require symbol resolution:
    // - completion
    // - completion_resolve
    // - hover
    // - goto_declaration
    // - goto_definition
    // - goto_type_definition
    // - goto_implementation
    // - references
    // - rename
    // - prepare_rename
    // - incoming_calls
    // - outgoing_calls
    // - prepare_call_hierarchy
    // - signature_help
    // - symbol
    //
    // These requests haven't been gotten around to yet:
    // - semantic_tokens_full_delta
    // - semantic_tokens_range
    // - semantic_tokens_refresh
    //
    // These need to be implemented as well:
    // - did_close, we probably need to react to the closing of documents somehow
    // - did_change_configuration
    // - did_change_watched_files
    // - did_change_workspace_folders
    // - did_save, this could be important?
    //
    // Other features:
    // - code_lens
    // - code_lens_resolve
    // - code_action
    // - code_action_resolve
    // - document_link
    // - document_link_resolve
    // - folding_range
    // - formatting, this is very complex so it's a low priority (use will_save_wait_until to format on save)
    // - execute_command, I'm sure there's some commands we can provide
}
