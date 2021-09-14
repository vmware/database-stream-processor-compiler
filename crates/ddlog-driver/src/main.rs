use ddlog_diagnostics::Interner;
use ddlog_lsp::{Backend, Session};
use ddlog_utils::Arc;
use lspower::{LspService, Server};
use tokio::{io, net::TcpListener};

#[tokio::main]
async fn main() {
    ddlog_driver::set_logger();

    // TODO: Error handling
    tracing::info!(address = "127.0.0.1:5007", "binding to tcp socket");
    let listener = TcpListener::bind("127.0.0.1:5007").await.unwrap();

    tracing::info!(address = "127.0.0.1:5007", "accepting tcp connections");
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = io::split(stream);

    tracing::info!("creating new ddlog backend");
    let session = Arc::new(Session::new(Interner::new()));
    let (service, messages) = LspService::new(|client| Backend::new(client, session));

    tracing::info!("spawning ddlog server");
    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}
