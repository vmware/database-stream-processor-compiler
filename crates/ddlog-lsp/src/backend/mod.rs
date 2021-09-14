mod language_server;
#[macro_use]
mod logging;

use crate::{
    database::{DDlogDatabase, Source},
    providers::utils,
    Session,
};
use cstree::TextRange;
use ddlog_diagnostics::{Diagnostic, FileId, Interner, Level};
use ddlog_utils::Arc;
use lspower::{
    lsp::{
        Diagnostic as LspDiagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location,
        MessageType, NumberOrString, Url,
    },
    Client,
};
use salsa::{ParallelDatabase, Snapshot};
use std::{fmt::Display, str::FromStr, sync::Mutex};

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

    pub fn snapshot(&self) -> Snapshot<DDlogDatabase> {
        self.database
            .lock()
            .expect("failed to lock database")
            .snapshot()
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
        Self::publish_diagnostics_raw(
            file,
            diagnostics,
            version,
            snapshot,
            self.session.interner(),
            &self.client,
        )
        .await;
    }

    pub async fn publish_diagnostics_raw(
        file: FileId,
        diagnostics: Vec<Diagnostic>,
        version: Option<i32>,
        snapshot: Snapshot<DDlogDatabase>,
        interner: &Interner,
        client: &Client,
    ) -> Snapshot<DDlogDatabase> {
        if diagnostics.is_empty() {
            tracing::debug!("was going to publish empty diagnostics bundle, abandoning");

            return snapshot;
        }

        let file_str = file.to_str(interner);
        tracing::debug!(
            file = file_str,
            version = ?version,
            total_diagnostics = diagnostics.len(),
            "publishing {} diagnostics",
            diagnostics.len(),
        );

        let uri = Url::from_str(file_str).unwrap();
        let source = snapshot.file_source(file);

        // TODO: Factor this conversion out into utility function(s)
        let diagnostics: Vec<_> = diagnostics
            .into_iter()
            .map(|diagnostic| {
                let primary_span = diagnostic.primary_span();

                let range = utils::ide_range(
                    &source,
                    TextRange::new(primary_span.start().into(), primary_span.end().into()),
                );
                let level = match diagnostic.level {
                    Level::Error => DiagnosticSeverity::Error,
                    Level::Warning => DiagnosticSeverity::Warning,
                    Level::Note => DiagnosticSeverity::Information,
                };
                let code = diagnostic
                    .code
                    .map(|code| NumberOrString::Number(code as i32));
                let labels: Vec<_> = diagnostic
                    .labels
                    .into_iter()
                    .map(|label| DiagnosticRelatedInformation {
                        message: label.message.unwrap().into_owned(),
                        location: Location {
                            uri: uri.clone(),
                            range: utils::ide_range(
                                &source,
                                TextRange::new(label.span.start().into(), label.span.end().into()),
                            ),
                        },
                    })
                    .collect();

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

        client.publish_diagnostics(uri, diagnostics, version).await;
        snapshot
    }
}
