use crate::utils::{
    checks,
    fs2::{self, display_path},
    npm, project_root,
};
use anyhow::{Context, Result};
use std::fs;

// TODO: Check that `rustc` and `cargo` exist and are >= our supported versions
// TODO: Check that `npm` exists and is >= our npm version
// TODO: Split into another file
// TODO: Ask to install git scripts
// TODO: Make git scripts
// TODO: Ask to install `vsce` globally
// TODO: Make sure that `rustfmt`, `clippy` and `mdbook` all exist and offer to
//       install them if they don't
pub fn setup() -> Result<()> {
    // Check that all required binaries exist
    checks::cargo_exists()?;
    checks::npm_exists()?;
    checks::rustfmt_exists()?;
    checks::clippy_exists()?;

    println!("installing vsce...");
    // Note: Don't immediately propagate the error, continue trying to do stuff
    let result = npm(&["install", "-g", "vsce"])
        .spawn()
        .context("failed to spawn `npm install -g vsce`")
        .and_then(|mut child| child.wait().context("failed to run `npm install -g vsce`"));

    let mut did_error = result.is_err();
    if let Ok(exit_code) = result {
        did_error = did_error || !exit_code.success();
    }

    if did_error {
        println!("failed");
    }
    result?;

    let vscode_path = project_root().join("editors/vscode");
    fs2::with_working_dir(&vscode_path, || {
        println!("initializing editor dependencies...");

        let result = npm(&["install"])
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
            println!("failed");
        }

        result
    })??;

    let hooks = project_root().join("etc/hooks");
    let git = project_root().join(".git/hooks");

    println!("installing git hooks to '{}'", display_path(&git));

    let entries = fs::read_dir(&hooks)
        .with_context(|| format!("failed to read the directory '{}'", hooks.display()))?
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().map(|ty| ty.is_file()).unwrap_or(false));

    // Copy over all the git hooks
    for entry in entries {
        let hook_path = git.join(entry.file_name());

        fs2::copy(entry.path(), &hook_path)?;
        println!("installed '{}'", display_path(hook_path));
    }

    Ok(())
}
