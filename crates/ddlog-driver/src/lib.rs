use tracing_subscriber::{fmt, EnvFilter};

/// Setup the lsp's logger to take log levels from the `DDLOG_LSP_LOG` env var, see
/// [`EnvFilter`] for more info on log directives
pub fn set_logger() {
    use tracing_subscriber::prelude::*;

    let env = EnvFilter::from_env("DDLOG_LSP_LOG");
    let formatter = fmt::layer().with_thread_names(true);

    tracing_subscriber::registry()
        .with(env)
        .with(formatter)
        .init();

    tracing::info!("logging hook has been set");
}
