use crate::utils::CodegenMode;
use anyhow::Result;
use gumdrop::Options;

mod codegen;
mod utils;

fn main() -> Result<()> {
    let options = XTaskOptions::parse_args_default_or_exit();

    match options {
        XTaskOptions::Help(XTaskHelp { command }) => {
            if let Some(command) = command {
                if let Some(usage) = XTaskOptions::command_usage(&command) {
                    println!("{}", usage);
                } else {
                    println!("could not find a command named '{}'", command);
                }
            } else {
                println!("{}", XTaskOptions::usage());
            }
        }

        XTaskOptions::Codegen(XTaskCodegen { mode }) => codegen::codegen(mode)?,
    }

    Ok(())
}

#[derive(Debug, Clone, Options)]
enum XTaskOptions {
    #[options(help = "show help for a command")]
    Help(XTaskHelp),

    #[options(help = "syntax and parser test code generation")]
    Codegen(XTaskCodegen),
    // TODO: xtask test --bless
}

#[derive(Debug, Clone, Options)]
struct XTaskHelp {
    #[options(
        free,
        help = "the command to get help for (if no command is given, help for the whole app will be displayed)"
    )]
    command: Option<String>,
}

#[derive(Debug, Clone, Options)]
struct XTaskCodegen {
    #[options(
        free,
        help = "(default: run) the mode to run codegen in, either 'run' or 'check'"
    )]
    pub mode: CodegenMode,
}
