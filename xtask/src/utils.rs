use anyhow::{Error, Result};
use std::{
    env,
    fmt::{self, Display},
    path::{Path, PathBuf},
    str::FromStr,
};

/// Normalizes the line endings of the given string
///
/// Replaces all instances of CRLF (`\r\n`) with LF (`\n`)
pub fn normalize_line_endings(string: &mut String) {
    unsafe {
        let vec = string.as_mut_vec();
        let ptr = vec.as_mut_ptr();

        let mut write = vec.as_mut_ptr();
        let mut read = vec.as_ptr();
        let end = read.add(vec.len());

        while read != end {
            if *read == b'\r' {
                let newline = read.add(1);
                if newline == end {
                    break;
                }

                if *newline == b'\n' {
                    read = newline;
                }
            }

            *write = *read;
            write = write.add(1);
            read = read.add(1);
        }

        vec.set_len(write as usize - ptr as usize);
    }
}

pub fn normalize_path_slashes(string: &mut String) {
    for byte in unsafe { string.as_bytes_mut() } {
        if *byte == b'\\' {
            *byte = b'/';
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodegenMode {
    Run,
    Check,
}

impl CodegenMode {
    /// Returns `true` if the codegen mode is [`Check`].
    ///
    /// [`Check`]: CodegenMode::Check
    #[allow(dead_code)]
    pub const fn is_check(self) -> bool {
        matches!(self, Self::Check)
    }
}

impl FromStr for CodegenMode {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.trim().to_lowercase().as_str() {
            "run" => Ok(Self::Run),
            "check" => Ok(Self::Check),

            _ => anyhow::bail!(
                "unrecognized codegen mode, expected 'run' or 'check', got '{:?}'",
                string.trim(),
            ),
        }
    }
}

impl Default for CodegenMode {
    fn default() -> Self {
        Self::Run
    }
}

impl Display for CodegenMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Run => "run",
            Self::Check => "check",
        })
    }
}

pub fn project_root() -> PathBuf {
    let manifest =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    Path::new(&manifest)
        .ancestors()
        .nth(1)
        .expect("failed to get ancestor of project root")
        .to_path_buf()
}

pub mod fs2 {
    use crate::utils::{normalize_line_endings, normalize_path_slashes, project_root, CodegenMode};
    use anyhow::{Context, Result};
    use std::{fs, path::Path};

    pub fn display_path<P>(path: P) -> String
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut pretty_path = path
            .strip_prefix(project_root())
            .unwrap_or(path)
            .display()
            .to_string();
        normalize_path_slashes(&mut pretty_path);
        if pretty_path.starts_with("../") {
            pretty_path.replace_range(.."../".len(), "");
        }

        pretty_path
    }

    /// Reads a file to string
    pub fn read_to_string<P>(path: P) -> Result<String>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))
    }

    /// Writes to a file
    pub fn write<P, C>(path: P, contents: C) -> Result<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        let path = path.as_ref();
        fs::write(path, contents)
            .with_context(|| format!("failed to write to '{}'", path.display()))
    }

    /// Creates a directory and all required parent directories
    pub fn create_dir_all<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        fs::create_dir_all(path)
            .with_context(|| format!("failed to create dir '{}'", path.display()))
    }

    /// A helper to update file on disk if it has changed
    pub fn update<P>(path: P, contents: &str, mode: CodegenMode) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let needs_update = if path.exists() {
            read_to_string(path)
                .map(|mut disk_contents| {
                    normalize_line_endings(&mut disk_contents);

                    let mut contents = contents.to_owned();
                    normalize_line_endings(&mut contents);

                    disk_contents != contents
                })
                .unwrap_or(true)
        } else {
            true
        };

        if needs_update {
            match mode {
                // If the file needs an update and we're in write mode, write to it
                CodegenMode::Run => {
                    // Make sure that the full path to the file exists
                    let parent_dir = path.parent().expect("files should have a parent path");
                    create_dir_all(parent_dir)?;

                    eprintln!("updating '{}'", display_path(&path));
                    write(path, contents)?;
                }

                // Otherwise throw an error since the file isn't updated
                CodegenMode::Check => anyhow::bail!("'{}' is not up-to-date", display_path(path)),
            }
        } else {
            eprintln!("up-to-date '{}'", display_path(path));
        }

        Ok(())
    }
}
