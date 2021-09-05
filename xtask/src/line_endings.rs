use crate::utils::{fs2, normalize_line_endings, project_root, CodegenMode};
use anyhow::Result;
use walkdir::WalkDir;

pub fn line_endings(mode: CodegenMode) -> Result<()> {
    println!(
        "{} line endings...",
        match mode {
            CodegenMode::Run => "normalizing",
            CodegenMode::Check => "checking",
        },
    );

    let root = project_root();

    let mut non_normalized = 0;
    for file in WalkDir::new(&root)
        .into_iter()
        .filter_entry(|entry| {
            entry.path() != root.join(".git")
                && entry.path() != root.join("target")
                && entry.path() != root.join("editors/vscode/node_modules")
                && entry.path() != root.join("editors/vscode/out")
                && entry.path() != root.join("editors/vscode/differential-datalog.vsix")
        })
        .flatten()
        .filter(|entry| entry.file_type().is_file())
    {
        let file = file.path();
        let mut contents = fs2::read_to_string(&file)?;

        if contents.contains("\r\n") {
            if mode.is_check() {
                eprintln!(
                    "file contains CRLF line endings: '{}'",
                    fs2::display_path(&file),
                );
                non_normalized += 1;
            } else {
                eprintln!(
                    "normalizing the line endings of '{}'",
                    fs2::display_path(&file),
                );

                normalize_line_endings(&mut contents);
                fs2::write(&file, &contents)?;
            }
        }
    }

    if mode.is_check() && non_normalized != 0 {
        anyhow::bail!(
            "{} files contained CRLF line endings, run `cargo xtask line-endings --mode run` to fix them",
        );
    }

    println!(
        "finished {} line endings",
        match mode {
            CodegenMode::Run => "normalizing",
            CodegenMode::Check => "checking",
        },
    );

    Ok(())
}
