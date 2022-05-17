use anyhow::{Context, Error, Result};
use std::{
    env,
    fmt::{self, Display},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
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

pub fn normalize_path_slashes(string: &mut str) {
    for byte in unsafe { string.as_bytes_mut() } {
        if *byte == b'\\' {
            *byte = b'/';
        }
    }
}

pub fn npm(args: &[&str]) -> Command {
    let mut command = if cfg!(windows) {
        Command::new("cmd.exe")
    } else {
        Command::new("npm")
    };

    if cfg!(windows) {
        command.args(&["/c", "npm"]).args(args);
    } else {
        command.args(args);
    }

    command
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

/// Formats rust code using `rustfmt`
pub fn format(code: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .args(&[
            "--edition",
            "2021",
            "--config",
            // TODO: Use `blank_lines_upper_bound=1` once it's stable
            "newline_style=Unix,normalize_doc_attributes=true",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to spawn rustfmt")?;

    {
        let mut stdin = child.stdin.take().context("failed to take rustfmt stdin")?;
        stdin
            .write_all(code.as_bytes())
            .context("failed to write to rustfmt stdin")?;
        stdin.flush().context("failed to flush rustfmt stdin")?;
    }

    let formatted = {
        let mut stdout = child
            .stdout
            .take()
            .context("failed to take rustfmt stdout")?;

        let mut buffer = String::with_capacity(code.len());
        stdout
            .read_to_string(&mut buffer)
            .context("failed to read from rustfmt stdout")?;

        buffer
    };

    let status = child.wait().context("failed to wait for rustfmt")?;
    if !status.success() {
        anyhow::bail!("rustfmt failed to run");
    }

    Ok(formatted)
}

pub mod fs2 {
    use crate::utils::{
        self, normalize_line_endings, normalize_path_slashes, project_root, CodegenMode,
    };
    use anyhow::{Context, Result};
    use std::{
        env, fs,
        panic::{self, UnwindSafe},
        path::Path,
    };

    pub fn with_working_dir<P, F, T>(path: P, with: F) -> Result<T>
    where
        P: AsRef<Path>,
        F: FnOnce() -> T + UnwindSafe,
    {
        let path = path.as_ref();

        let current_dir = env::current_dir().context("the current working directory is invalid")?;
        env::set_current_dir(path).with_context(|| {
            format!(
                "failed to set working directory to '{}'",
                display_path(path),
            )
        })?;

        let result = panic::catch_unwind(with)
            .map_err(|err| anyhow::format_err!("function panicked with working dir: {:?}", err));

        env::set_current_dir(current_dir).context("failed to reset working directory")?;

        result
    }

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

    /// Removes a file
    pub fn remove_file<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        fs::remove_file(path).with_context(|| format!("failed to remove '{}'", path.display()))
    }

    const GENERATED_CODE_HEADER: &str = "// This code is generated by xtask, do not manually edit. Run `cargo xtask codegen` to re-generate";

    /// Update a file after running `contents` through `rustfmt`
    pub fn update_formatted<P>(path: P, contents: &str, mode: CodegenMode) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut final_contents =
            String::with_capacity(GENERATED_CODE_HEADER.len() + 2 + contents.len());
        final_contents.push_str(GENERATED_CODE_HEADER);
        final_contents.push_str("\n\n");
        final_contents.push_str(contents);

        let formatted = utils::format(&final_contents)?;
        update(path, &formatted, mode)?;

        Ok(())
    }

    /// A helper to update file on disk if it has changed
    pub fn update<P>(path: P, contents: &str, mode: CodegenMode) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let mut contents = contents.to_owned();
        normalize_line_endings(&mut contents);

        let needs_update = if path.exists() {
            read_to_string(path)
                .map(|disk_contents| disk_contents != contents)
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

                    println!("updating '{}'", display_path(&path));
                    write(path, contents)?;
                }

                // Otherwise throw an error since the file isn't updated
                CodegenMode::Check => anyhow::bail!("'{}' is not up-to-date", display_path(path)),
            }
        } else {
            println!("up-to-date '{}'", display_path(path));
        }

        Ok(())
    }

    pub fn copy<P1, P2>(from: P1, to: P2) -> Result<()>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let (from, to) = (from.as_ref(), to.as_ref());

        if !from.exists() {
            let from = from.display();
            anyhow::bail!(
                "cannot copy from '{}' to '{}', '{}' doesn't exist",
                from,
                to.display(),
                from,
            )
        }

        // Create the path to the target file
        let parent_dir = to.parent().expect("files should have a parent path");
        create_dir_all(parent_dir)?;

        // Copy the file
        fs::copy(from, to).with_context(|| {
            format!(
                "failed to copy from '{}' to '{}'",
                from.display(),
                to.display(),
            )
        })?;

        Ok(())
    }
}

pub mod ansi {
    pub const YELLOW: &str = "\x1B[1;33m";
    // pub const GREEN: &str = "\x1B[1;32m";
    // pub const RED: &str = "\x1B[1;31m";
    pub const RESET: &str = "\x1B[0m";
}

pub mod checks {
    use crate::utils::npm;
    use anyhow::{Context, Result};
    use std::process::Command;

    /// This will probably never be used, but just in case
    const MIN_MAJOR_RUST_VERSION: usize = 1;
    /// The latest stable Rust version, feel free to open PRs to update it
    const MIN_MINOR_RUST_VERSION: usize = 57;

    pub fn cargo_exists() -> Result<()> {
        let output = Command::new("cargo")
            .arg("--version")
            .output()
            .context("`cargo` is required for building ddlog")?;

        // Parse out the semver major and minor versions of cargo
        // ```
        // cargo 1.55.0 (32da73ab1 2021-08-23)
        //       ^ ^^
        // ```
        let version = String::from_utf8(output.stdout).context("cargo didn't return valid utf8")?;
        let (major, minor): (usize, usize) = version
            .trim()
            .split(' ')
            .nth(1)
            .and_then(|version| {
                let mut split = version.split('.');
                Some((split.next()?, split.next()?))
            })
            .and_then(|(major, minor)| Some((major.parse().ok()?, minor.parse().ok()?)))
            .with_context(|| format!("failed to parse cargo's output: {:?}", version))?;

        if major < MIN_MAJOR_RUST_VERSION || minor < MIN_MINOR_RUST_VERSION {
            anyhow::bail!(
                "You have a pre-{major}.{minor} version of cargo installed, please update to at least {major}.{minor}",
                major = MIN_MAJOR_RUST_VERSION,
                minor = MIN_MINOR_RUST_VERSION,
            );
        }

        println!(
            "detected {} (>= minimum of {}.{})",
            version.trim(),
            MIN_MAJOR_RUST_VERSION,
            MIN_MINOR_RUST_VERSION,
        );

        Ok(())
    }

    pub fn rustfmt_exists() -> Result<()> {
        let output = Command::new("rustfmt").arg("--version").output().context(
            "`rustfmt` is required for code generation, try running `rustup component add rustfmt`",
        )?;

        if !output.status.success() {
            anyhow::bail!("`rustfmt` is required for code generation, try running `rustup component add rustfmt`");
        }

        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("detected {}", version.trim());
        }

        Ok(())
    }

    pub fn clippy_exists() -> Result<()> {
        let output = Command::new("cargo")
            .args(&["clippy", "--", "--version"])
            .output()
            .context("`clippy` is required for code generation, try running `rustup component add clippy`")?;

        if !output.status.success() {
            anyhow::bail!("`clippy` is required for code generation, try running `rustup component add clippy`");
        }

        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("detected {}", version.trim());
        }

        Ok(())
    }

    pub fn npm_exists() -> Result<()> {
        let output = npm(&["--version"])
            .output()
            .context("`npm` is required to build the VS Code plugin")?;

        if !output.status.success() {
            anyhow::bail!("`npm` is required to build the VS Code plugin");
        }

        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("detected npm {}", version.trim());
        }

        Ok(())
    }
}
