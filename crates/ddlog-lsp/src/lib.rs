use dashmap::DashMap;
use ddlog_diagnostics::{FileId, Interner};
use lsp_text::RopeExt;
use lspower::{jsonrpc::Result, lsp::*, Client, LanguageServer};
use ropey::Rope;
use serde_json::Value;
use std::fmt::Display;
use triomphe::Arc;

#[derive(Debug)]
pub struct Session {
    interner: Interner,
    files: DashMap<FileId, Rope>,
}

impl Session {
    pub fn new(interner: Interner) -> Self {
        Self {
            interner,
            files: DashMap::new(),
        }
    }
}

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

    pub async fn error<M>(&self, message: M)
    where
        M: Display,
    {
        self.client.log_message(MessageType::Error, message).await;
    }
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::Incremental,
            )),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec![".".to_string()]),
                ..Default::default()
            }),
            execute_command_provider: Some(ExecuteCommandOptions {
                commands: vec!["dummy.do_something".to_string()],
                ..Default::default()
            }),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                ..Default::default()
            }),
            ..ServerCapabilities::default()
        };

        Ok(InitializeResult {
            server_info: None,
            capabilities,
        })
    }

    async fn initialized(&self, initialized: InitializedParams) {
        self.info(format!("initialized: {:?}", initialized)).await;
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

        if opened.text_document.language_id == "ddlog" {
            let file_id = FileId::new(self.session.interner.get_or_intern(file_name));

            self.session
                .files
                .insert(file_id, Rope::from(opened.text_document.text));
        }
    }

    async fn did_change(&self, changes: DidChangeTextDocumentParams) {
        let file_name = changes.text_document.uri.as_str();
        let file_id = FileId::new(self.session.interner.get_or_intern(file_name));

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
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.info("file saved!").await;
    }

    async fn did_close(&self, closed: DidCloseTextDocumentParams) {
        let file_name = closed.text_document.uri.as_str();
        let file_id = FileId::new(self.session.interner.get_or_intern(file_name));

        self.info(format!("file closed: {}", file_name)).await;

        self.session.files.remove(&file_id);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}
