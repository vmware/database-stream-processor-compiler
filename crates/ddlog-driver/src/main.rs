use ddlog_diagnostics::Interner;
use ddlog_lsp::{Backend, Session};
use lspower::{LspService, Server};
use tokio::{io, net::TcpListener};
use triomphe::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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
