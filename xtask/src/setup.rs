use crate::utils::{
    fs2::{self, display_path},
    project_root,
};
use anyhow::{Context, Result};
use std::process::Command;

// TODO: Check that `rustc` and `cargo` exist and are >= our supported versions
// TODO: Check that `npm` exists and is >= our npm version
// TODO: Split into another file
// TODO: Ask to install git scripts
// TODO: Make git scripts
// TODO: Ask to install `vsce` globally
// TODO: Make sure that `rustfmt`, `clippy` and `mdbook` all exist and offer to
//       install them if they don't
pub fn setup() -> Result<()> {
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
        did_error = did_error || !exit_code.success();
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

    Ok(())
}
