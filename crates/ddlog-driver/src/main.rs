use ddlog_syntax::{FileId, Interner, NodeCache};
use std::{
    collections::HashMap,
    io::{self, Write},
};

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
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    let interner = Interner::new();
    let mut cache_interner = interner.clone();
    let mut cache = NodeCache::with_interner(&mut cache_interner);
    let file = FileId::new(interner.get_or_intern_static("repl/input.dl"));

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let trimmed = input.trim();
        if trimmed == ":exit" {
            println!("exiting...");
            break;
        } else if trimmed == ":help" {
            println!("{}", HELP.trim());
            input.clear();
            continue;
        }

        let (root, errors) = if trimmed.starts_with(EXPR_HEADER) {
            input.replace_range(..EXPR_HEADER.len(), "");
            ddlog_syntax::parse_expr(file, &input, &mut cache)
        } else {
            if trimmed.starts_with(ITEM_HEADER) {
                input.replace_range(..ITEM_HEADER.len(), "");
            }

            ddlog_syntax::parse(file, &input, &mut cache)
        };

        println!("{}", root.debug(&interner, true));
        if !errors.is_empty() {
            struct FileCache(ariadne::Source, Interner);

            impl ariadne::Cache<FileId> for FileCache {
                fn fetch(
                    &mut self,
                    _id: &FileId,
                ) -> Result<&ariadne::Source, Box<dyn std::fmt::Debug + '_>> {
                    Ok(&self.0)
                }

                fn display<'a>(&self, &id: &'a FileId) -> Option<Box<dyn std::fmt::Display + 'a>> {
                    Some(Box::new(self.1.resolve(id.file_name()).to_owned()))
                }
            }

            let mut file_cache = FileCache(ariadne::Source::from(input.clone()), interner.clone());
            for error in errors {
                error.eprint(&mut file_cache).unwrap();
            }
        }

        input.clear();
    }

    Ok(())
}
