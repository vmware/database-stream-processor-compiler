mod codegen;
mod line_endings;
mod setup;
mod utils;

use crate::utils::CodegenMode;
use anyhow::Result;
use gumdrop::Options;

// TODO: Add a `test` command to run file tests, rust tests and editor tests
// TODO: Add an `install` command to build and install the lsp and extension
// TODO: Add a command to build/publish the mdbook
fn main() -> Result<()> {
    let options = XTaskOptions::parse_args_default_or_exit();

    match options {
        XTaskOptions::Help(Help { command }) => {
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

        XTaskOptions::Codegen(Codegen { mode }) => codegen::codegen(mode)?,

        XTaskOptions::LineEndings(LineEndings { mode }) => line_endings::line_endings(mode)?,

        XTaskOptions::Setup(Setup {}) => setup::setup()?,
    }

    Ok(())
}

#[derive(Debug, Clone, Options)]
enum XTaskOptions {
    #[options(help = "show help for a command")]
    Help(Help),

    #[options(help = "syntax and parser test code generation")]
    Codegen(Codegen),

    #[options(help = "setup your local environment")]
    Setup(Setup),

    #[options(help = "check and normalize the line endings of the repo")]
    LineEndings(LineEndings),
}

#[derive(Debug, Clone, Options)]
struct Help {
    #[options(
        free,
        help = "the command to get help for (if no command is given, help for the whole app will be displayed)"
    )]
    command: Option<String>,
}

#[derive(Debug, Clone, Options)]
struct Codegen {
    #[options(
        free,
        help = "(default: run) the mode to run codegen in, either 'run' or 'check'"
    )]
    pub mode: CodegenMode,
}

#[derive(Debug, Clone, Options)]
struct Setup {}

#[derive(Debug, Clone, Options)]
struct LineEndings {
    #[options(
        default = "check",
        help = "(default: check) the mode to run codegen in, either 'run' or 'check'"
    )]
    pub mode: CodegenMode,
}
