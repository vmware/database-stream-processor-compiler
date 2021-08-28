use ddlog_syntax::{FileId, Interner, NodeCache};
use std::io::{self, Write};

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

        let (root, _errors) = ddlog_syntax::parse(file, &input, &mut cache);
        println!("{}", root.debug(&interner, true));

        input.clear();
    }
}
