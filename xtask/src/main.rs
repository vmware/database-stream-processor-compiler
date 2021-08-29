use anyhow::Result;
use gumdrop::Options;

mod codegen;
mod utils;

fn main() -> Result<()> {
    let options = XTaskOptions::parse_args_default_or_exit();

    match options {
        XTaskOptions::Help(_) => println!("{}", XTaskOptions::usage()),
        XTaskOptions::Codegen(_) => codegen::codegen()?,
    }

    Ok(())
}

#[derive(Debug, Clone, Options)]
enum XTaskOptions {
    #[options(help = "show help for a command")]
    Help(XTaskHelp),
    #[options(help = "run codegen")]
    Codegen(XTaskCodegen),
}

#[derive(Debug, Clone, Options)]
struct XTaskHelp {}

#[derive(Debug, Clone, Options)]
struct XTaskCodegen {}
