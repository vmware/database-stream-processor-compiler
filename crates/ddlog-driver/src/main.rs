// /*
use ddlog_diagnostics::Interner;
use ddlog_lsp::{Backend, Session};
use lspower::{LspService, Server};
use tokio::io;
use triomphe::Arc;

#[tokio::main]
async fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());

    let session = Arc::new(Session::new(Interner::new()));
    let (service, messages) = LspService::new(|client| Backend::new(client, session));

    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}
// */
/*
use ddlog_diagnostics::{DiagnosticConfig, FileCache, FileId, Interner};
use ddlog_syntax::NodeCache;
use std::io::{self, Write};

const EXPR_HEADER: &str = ":expr ";
const ITEM_HEADER: &str = ":item ";

const HELP: &str = "
DDlog Repl

COMMANDS:
  :help           Show this message
  :item <item>    Parse the given item
  :expr <expr>    Parse the given expr
  :exit           Exit the repl
";

fn main() -> io::Result<()> {
    let (stdin, mut stdout) = (io::stdin(), io::stdout());
    let mut input = String::new();

    let interner = Interner::new();
    let diagnostic_config = DiagnosticConfig::new();

    let mut cache_interner = interner.clone();
    let mut cache = Some(NodeCache::with_interner(&mut cache_interner));

    let mut file_cache = FileCache::new(interner.clone());
    let file = FileId::new(interner.get_or_intern_static("repl/input.dl"));

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        input = input.trim().to_owned();
        if input == ":exit" {
            println!("exiting...");
            break;
        } else if input == ":help" {
            println!("{}", HELP.trim());
            input.clear();
            continue;
        }

        let parse_cache = cache.take().unwrap();
        let (parsed, parse_cache) = if input.starts_with(EXPR_HEADER) {
            input.replace_range(..EXPR_HEADER.len(), "");
            ddlog_syntax::parse_expr(file, &input, parse_cache)
        } else {
            if input.starts_with(ITEM_HEADER) {
                input.replace_range(..ITEM_HEADER.len(), "");
            }

            ddlog_syntax::parse(file, &input, parse_cache)
        };
        cache = Some(parse_cache);

        println!("{}", parsed.debug_tree());
        if parsed.has_errors() {
            file_cache.add_str(file, &input);

            for error in parsed.into_errors() {
                error
                    .emit_to(&diagnostic_config, &mut file_cache, &mut stdout)
                    .unwrap();
            }
        }

        input.clear();
        file_cache.clear();
    }

    Ok(())
}
*/
