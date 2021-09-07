use ddlog_diagnostics::{DiagnosticConfig, FileCache, FileId, Interner, Rope};
use ddlog_syntax::{ast::AstNode, validation, NodeCache, RuleCtx};
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
    tracing_subscriber::fmt::init();

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
        let (mut parsed, parse_cache) = if input.starts_with(EXPR_HEADER) {
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

        let mut ctx = RuleCtx::new(file, Rope::from_str(&input), interner.clone());
        ctx.diagnostics.extend(parsed.take_errors());

        validation::run_validators(parsed.root().syntax(), &mut ctx);

        if !ctx.diagnostics.is_empty() {
            file_cache.add_str(file, &input);

            for error in ctx.diagnostics {
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
