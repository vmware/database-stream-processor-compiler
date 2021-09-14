use ddlog_diagnostics::Interner;
use ddlog_lsp::{Backend, Session};
use ddlog_utils::Arc;
use lspower::{LspService, Server};
use tokio::{io, net::TcpListener};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    set_logger();

    // TODO: Error handling
    let listener = TcpListener::bind("127.0.0.1:5007").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = io::split(stream);

    let session = Arc::new(Session::new(Interner::new()));
    let (service, messages) = LspService::new(|client| Backend::new(client, session));

    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}

/// Setup the lsp's logger to take log levels from the `DDLOG_LSP_LOG` env var, see
/// [`EnvFilter`] for more info on log directives
fn set_logger() {
    use tracing_subscriber::prelude::*;

    let env = EnvFilter::from_env("DDLOG_LSP_LOG");
    let formatter = fmt::layer().with_thread_names(true);

    tracing_subscriber::registry()
        .with(env)
        .with(formatter)
        .init();
}
