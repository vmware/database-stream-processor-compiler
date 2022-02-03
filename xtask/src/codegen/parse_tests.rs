//! Takes comments from ddlog-syntax and turns them into test data.
//! This code is derived from rust_analyzer/xtask/codegen/gen_parser_tests

use crate::{
    utils::{
        ansi::{RESET, YELLOW},
        fs2::{self, display_path, update},
        project_root, CodegenMode,
    },
    Result,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    fs, iter, mem,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn parse_tests(mode: CodegenMode) -> Result<()> {
    match mode {
        CodegenMode::Run => println!("running test generation..."),
        CodegenMode::Check => println!("checking generated tests..."),
    }

    let mut missing_dumps = false;

    let parser_dir = project_root().join("crates/ddlog-syntax/src/parser");
    let parse_tests = tests_from_dir(&parser_dir, false)?;

    if install_tests(
        &parse_tests.pass,
        "crates/ddlog-syntax/tests/parse/pass",
        mode,
    )? {
        missing_dumps = true
    }
    if install_tests(
        &parse_tests.fail,
        "crates/ddlog-syntax/tests/parse/fail",
        mode,
    )? {
        missing_dumps = true;
    }

    let validation_dir = project_root().join("crates/ddlog-syntax/src/validation");
    let validation_tests = tests_from_dir(&validation_dir, true)?;

    if install_tests(
        &validation_tests.pass,
        "crates/ddlog-syntax/tests/validation/pass",
        mode,
    )? {
        missing_dumps = true
    }
    if install_tests(
        &validation_tests.fail,
        "crates/ddlog-syntax/tests/validation/fail",
        mode,
    )? {
        missing_dumps = true;
    }

    if missing_dumps {
        println!(
            "{}warning{}: missing dump files, run `cargo test` with `UPDATE_EXPECT` set to 1",
            YELLOW, RESET,
        );
        println!(
            "{}warning{}:     shell: `UPDATE_EXPECT=1 cargo test`",
            YELLOW, RESET,
        );
        println!(
            "{}warning{}:     cmd: `set UPDATE_EXPECT=1 && cargo test && set UPDATE_EXPECT=`",
            YELLOW, RESET,
        );
        println!(
            "{}warning{}:     powershell: `$env:UPDATE_EXPECT=1; cargo test; Remove-Item Env:\\UPDATE_EXPECT`",
            YELLOW, RESET,
        );
    }
    if missing_dumps && mode.is_check() {
        anyhow::bail!("missing dump files");
    }

    match mode {
        CodegenMode::Run => println!("finished running test generation"),
        CodegenMode::Check => println!("finished checking generated tests"),
    }

    Ok(())
}

fn install_tests(tests: &HashMap<String, Test>, test_dir: &str, mode: CodegenMode) -> Result<bool> {
    let tests_dir = project_root().join(test_dir);
    if !tests_dir.is_dir() {
        fs2::create_dir_all(&tests_dir)?;
    }

    // ok is never actually read, but it needs to be specified to create a Test in existing_tests
    let existing = existing_tests(&tests_dir, true)?;
    if let Some(test) = existing.keys().find(|&test| !tests.contains_key(test)) {
        anyhow::bail!("test was deleted: {}", test);
    }

    for (name, test) in tests {
        let path = match existing.get(name) {
            Some((path, _)) => path.clone(),
            None => tests_dir.join(format!("{}.dl", name)),
        };

        update(&path, &test.code, mode)?;
    }

    let test_paths: HashSet<_> = tests
        .values()
        .flat_map(|test| {
            iter::once(tests_dir.join(format!("{}.dl", test.name)))
                .chain(iter::once(tests_dir.join(format!("{}.rast", test.name))))
        })
        .chain(iter::once(tests_dir.clone()))
        .collect();

    // Delete any extra files
    for entry in WalkDir::new(&tests_dir).into_iter().flatten() {
        if !test_paths.contains(entry.path()) {
            match mode {
                CodegenMode::Run => {
                    println!("removing '{}'", display_path(entry.path()));
                    fs2::remove_file(entry.path())?;
                }

                CodegenMode::Check => {
                    anyhow::bail!("excess file in test dir: '{}'", display_path(entry.path()));
                }
            }
        }
    }

    let mut missing_dumps = false;
    for (name, test) in tests {
        let dump_file = tests_dir.join(format!("{}.rast", test.name));

        if !dump_file.exists() {
            println!(
                "{}warning{}: the dump file associated with test '{}' doesn't exist (dump file: '{}')",
                YELLOW, RESET,
                name,
                display_path(dump_file),
            );

            missing_dumps = true;
        }
    }

    Ok(missing_dumps)
}

fn extract_comment_blocks(
    text: &str,
    allow_blocks_with_empty_lines: bool,
) -> Vec<(Location, Vec<&str>)> {
    let mut res = Vec::new();

    let prefix = "// - ";

    let mut block = (Location::default(), vec![]);
    for (line_num, line) in text.lines().map(str::trim_start).enumerate() {
        if line == "//" && allow_blocks_with_empty_lines {
            block.1.push("");
            continue;
        }

        let (is_header, is_comment) = (line.starts_with("// test"), line.starts_with(prefix));
        if is_header {
            if !block.1.is_empty() {
                res.push(mem::take(&mut block));
            }

            block.1.push(&line["// ".len()..]);
            block.0 = Location::new(
                line_num as u32,
                (line.len() - line["// ".len()..].len()) as u32,
            );
        } else if is_comment {
            block.1.push(&line[prefix.len()..]);
        } else if !block.1.is_empty() {
            res.push(mem::take(&mut block));
        }
    }

    if !block.1.is_empty() {
        res.push(block)
    }

    res
}

#[derive(Debug)]
struct Test {
    pub name: String,
    pub code: String,
    pub pass: bool,
}

#[derive(Debug)]
enum TestKind {
    Item,
    Stmt,
    Expr,
}

#[derive(Default, Debug)]
struct Tests {
    pub pass: HashMap<String, Test>,
    pub fail: HashMap<String, Test>,
}

// TODO: Allow for giving `:expr`/`:stmt`/`:item` specifiers in tests
fn collect_tests(source: &str, all_validate: bool, src_file: &Path) -> Vec<Test> {
    let mut tests = Vec::new();
    for (location, comment_block) in extract_comment_blocks(source, false) {
        let first_line = &comment_block[0];
        let (line, pass) = if let Some(line) = first_line.strip_prefix("test_err") {
            (line.trim(), false)
        } else if let Some(line) = first_line.strip_prefix("test") {
            (line.trim(), true)
        } else {
            continue;
        };
        let ignore = false;

        let (name, kind) = if let Some(name) = line.strip_prefix("(item) ") {
            (name, TestKind::Item)
        } else if let Some(name) = line.strip_prefix("(stmt) ") {
            (name, TestKind::Stmt)
        } else if let Some(name) = line.strip_prefix("(expr) ") {
            (name, TestKind::Expr)
        } else {
            (line, TestKind::Item)
        };

        let pretty_file = src_file
            .strip_prefix(&project_root().join("crates"))
            .unwrap_or(src_file);
        let header = format!(
            "// kind:{} validate:{} pass:{} ignore:{} file:'{}' line:{} column:{}",
            match kind {
                TestKind::Item => "item",
                TestKind::Stmt => "stmt",
                TestKind::Expr => "expr",
            },
            all_validate,
            pass,
            ignore,
            display_path(pretty_file),
            location.line,
            location.column,
        );
        let code = iter::once(&*header)
            .chain(comment_block[1..].iter().map(|line| &**line))
            .chain(iter::once(""))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!name.trim().is_empty());
        assert!(!code.trim().is_empty() && code.ends_with('\n'));
        tests.push(Test {
            name: name.trim().to_owned(),
            code,
            pass,
        })
    }

    tests
}

fn tests_from_dir(dir: &Path, all_validate: bool) -> Result<Tests> {
    let mut tests = Tests::default();
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() || entry.path().extension().unwrap_or_default() != "rs" {
            continue;
        }

        process_file(&mut tests, entry.path(), all_validate)?;
    }

    let total = tests.pass.len() + tests.fail.len();
    println!(
        "found {} inline test{}",
        total,
        if total == 1 { "" } else { "s" },
    );

    Ok(tests)
}

fn process_file(res: &mut Tests, path: &Path, all_validate: bool) -> Result<()> {
    let text = fs::read_to_string(path)?;
    let mut duplicate_tests = Vec::new();

    for test in collect_tests(&text, all_validate, path) {
        if test.pass {
            if let Some(old_test) = res.pass.insert(test.name.clone(), test) {
                duplicate_tests.push(old_test.name);
            }
        } else if let Some(old_test) = res.fail.insert(test.name.clone(), test) {
            duplicate_tests.push(old_test.name);
        }
    }

    if !duplicate_tests.is_empty() {
        let total_duplicates = duplicate_tests.len();
        let duplicates = if total_duplicates == 1 {
            duplicate_tests.into_iter().next().unwrap()
        } else {
            duplicate_tests.sort_unstable();

            let mut duplicates = String::new();
            for test in duplicate_tests {
                write!(&mut duplicates, "- {}", test).unwrap();
            }

            duplicates
        };

        anyhow::bail!(
            "{} duplicate test{}: {}",
            total_duplicates,
            if total_duplicates == 1 { "" } else { "s" },
            duplicates,
        );
    }

    Ok(())
}

fn existing_tests(dir: &Path, ok: bool) -> Result<HashMap<String, (PathBuf, Test)>> {
    let mut res = HashMap::new();
    for file in fs::read_dir(dir)? {
        let file = file?;
        let path = file.path();

        if path.extension().unwrap_or_default() != "rs" {
            continue;
        }

        let name = {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            file_name[5..file_name.len() - 3].to_string()
        };

        let text = fs::read_to_string(&path)?;
        let test = Test {
            name: name.clone(),
            code: text,
            pass: ok,
        };

        if let Some(old) = res.insert(name, (path, test)) {
            println!("duplicate test: {:?}", old);
        }
    }

    Ok(res)
}

#[derive(Debug, Default)]
struct Location {
    line: u32,
    column: u32,
}

impl Location {
    fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}
