use crate::utils::{
    fs2::{self, display_path},
    project_root, CodegenMode,
};
use anyhow::{Context, Result};
use gumdrop::Options;
use std::process::Command;

mod codegen;
mod line_endings;
mod utils;

// TODO: Add a `test` command to run file tests, rust tests and editor tests
// TODO: Add an `install` command to build and install the lsp and extension
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

        // TODO: Check that `rustc` and `cargo` exist and are >= our supported versions
        // TODO: Check that `npm` exists and is >= our npm version
        // TODO: Split into another file
        // TODO: Ask to install git scripts
        // TODO: Make git scripts
        // TODO: Ask to install `vsce` globally
        // TODO: Something's funky with running npm install?
        XTaskOptions::Setup(Setup {}) => {
            // npm uses a cmd command on windows
            let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };

            eprint!("installing vsce... ");
            // Note: Don't immediately propagate the error, continue trying to do stuff
            let result = Command::new(npm)
                .args(&["install", "-g", "vsce"])
                .spawn()
                .context("failed to spawn `npm install -g vsce`")
                .and_then(|mut child| child.wait().context("failed to run `npm install -g vsce`"));

            let mut did_error = result.is_err();
            if let Ok(exit_code) = result {
                did_error = !exit_code.success();
            }
            if did_error {
                eprintln!("failed");
            } else {
                eprintln!("done");
            }

            let vscode_path = project_root().join("editors/vscode");
            fs2::with_working_dir(&vscode_path, || {
                eprint!("initializing editor dependencies... ");

                let result = Command::new(npm)
                    .arg("install")
                    .spawn()
                    .and_then(|mut child| child.wait())
                    .with_context(|| {
                        format!(
                            "failed to run `npm install` in '{}'",
                            display_path(&vscode_path),
                        )
                    });

                let mut did_error = result.is_err();
                if let Ok(exit_code) = result {
                    did_error = !exit_code.success();
                }
                if did_error {
                    eprintln!("failed");
                } else {
                    eprintln!("done");
                }

                result
            })??;
            result?;
        }
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
