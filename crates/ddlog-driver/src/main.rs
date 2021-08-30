/*
use ddlog_lsp::{Backend, Session};
use ddlog_syntax::Interner;
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
*/

use ddlog_diagnostics::{FileCache, FileId, Interner};
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

    let mut cache_interner = interner.clone();
    let mut cache = NodeCache::with_interner(&mut cache_interner);

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

        let (root, errors) = if input.starts_with(EXPR_HEADER) {
            input.replace_range(..EXPR_HEADER.len(), "");
            ddlog_syntax::parse_expr(file, &input, &mut cache)
        } else {
            if input.starts_with(ITEM_HEADER) {
                input.replace_range(..ITEM_HEADER.len(), "");
            }

            ddlog_syntax::parse(file, &input, &mut cache)
        };

        println!("{}", root.debug(&interner, true));
        if !errors.is_empty() {
            file_cache.add_str(file, &input);

            for error in errors {
                error.emit_to(&mut file_cache, &mut stdout).unwrap();
            }
        }

        input.clear();
        file_cache.clear();
    }

    Ok(())
}
