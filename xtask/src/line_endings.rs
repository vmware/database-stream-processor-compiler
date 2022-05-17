use crate::utils::{fs2, normalize_line_endings, project_root, CodegenMode};
use anyhow::Result;
use walkdir::WalkDir;

const IGNORED_DIRECTORIES: &[&str] = &[
    ".git",
    "target",
    "editors/vscode/node_modules",
    "editors/vscode/out",
];
const IGNORED_EXTENSIONS: &[&str] = &["vsix"];

pub fn line_endings(mode: CodegenMode) -> Result<()> {
    println!(
        "{} line endings...",
        match mode {
            CodegenMode::Run => "normalizing",
            CodegenMode::Check => "checking",
        },
    );

    let root = project_root();
    let ignored_directories: Vec<_> = IGNORED_DIRECTORIES
        .iter()
        .map(|&dir| root.join(dir))
        .collect();

    let mut non_normalized = 0;
    for file in WalkDir::new(&root)
        .into_iter()
        .filter_entry(|entry| {
            ignored_directories
                .iter()
                .all(|ignored| entry.path() != ignored)
                && entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(true, |ext| !IGNORED_EXTENSIONS.contains(&ext))
        })
        .flatten()
        .filter(|entry| entry.file_type().is_file())
    {
        let file = file.path();
        let mut contents = fs2::read_to_string(&file)?;

        if contents.contains("\r\n") {
            if mode.is_check() {
                println!(
                    "file contains CRLF line endings: '{}'",
                    fs2::display_path(&file),
                );
                non_normalized += 1;
            } else {
                println!(
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
            "{non_normalized} files contained CRLF line endings, run \
            `cargo xtask line-endings --mode run` to fix them",
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
