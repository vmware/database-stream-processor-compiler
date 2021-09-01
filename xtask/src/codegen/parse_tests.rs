//! Takes comments from ddlog-syntax and turns them into test data.
//! This code is derived from rust_analyzer/xtask/codegen/gen_parser_tests

use crate::{
    utils::{
        fs2::{self, update},
        project_root, CodegenMode,
    },
    Result,
};
use std::{
    collections::HashMap,
    fs, iter, mem,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn parse_tests(mode: CodegenMode) -> Result<()> {
    match mode {
        CodegenMode::Run => eprintln!("running test generation..."),
        CodegenMode::Check => eprintln!("checking generated tests..."),
    }

    let parser_dir = project_root().join("crates/ddlog-syntax/src/parser");
    let tests = tests_from_dir(&parser_dir)?;
    install_tests(&tests.pass, "crates/ddlog-syntax/tests/inline/pass", mode)?;
    install_tests(&tests.fail, "crates/ddlog-syntax/tests/inline/fail", mode)?;

    match mode {
        CodegenMode::Run => eprintln!("finished running tests generation"),
        CodegenMode::Check => eprintln!("finished checking generated tests"),
    }

    Ok(())
}

fn install_tests(tests: &HashMap<String, Test>, test_dir: &str, mode: CodegenMode) -> Result<()> {
    let tests_dir = project_root().join(test_dir);
    if !tests_dir.is_dir() {
        fs2::create_dir_all(&tests_dir)?;
    }

    // ok is never actually read, but it needs to be specified to create a Test in existing_tests
    let existing = existing_tests(&tests_dir, true)?;
    for test in existing.keys().filter(|&test| !tests.contains_key(test)) {
        panic!("test is deleted: {}", test);
    }

    for (name, test) in tests {
        let path = match existing.get(name) {
            Some((path, _)) => path.clone(),
            None => {
                let file_name = format!("{}.dl", name);
                tests_dir.join(file_name)
            }
        };

        update(&path, &test.code, mode)?;
    }

    Ok(())
}

fn extract_comment_blocks(
    text: &str,
    allow_blocks_with_empty_lines: bool,
) -> Vec<(usize, Vec<&str>)> {
    let mut res = Vec::new();

    let prefix = "// - ";

    let mut block = (0, vec![]);
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
        } else if is_comment {
            block.1.push(&line[prefix.len()..]);
        } else {
            if !block.1.is_empty() {
                res.push(mem::take(&mut block));
            }

            block.0 = line_num + 2;
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
    pub kind: TestKind,
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
fn collect_tests(s: &str) -> Vec<Test> {
    let mut tests = Vec::new();
    for comment_block in extract_comment_blocks(s, false).into_iter().map(|(_, x)| x) {
        let first_line = &comment_block[0];
        let (line, pass) = if let Some(line) = first_line.strip_prefix("test_err") {
            (line.trim(), false)
        } else if let Some(line) = first_line.strip_prefix("test") {
            (line.trim(), true)
        } else {
            continue;
        };

        let (name, kind) = if let Some(name) = line.strip_prefix("(item) ") {
            (name, TestKind::Item)
        } else if let Some(name) = line.strip_prefix("(stmt) ") {
            (name, TestKind::Stmt)
        } else if let Some(name) = line.strip_prefix("(expr) ") {
            (name, TestKind::Expr)
        } else {
            (line, TestKind::Item)
        };

        let header = format!(
            "// kind:{}",
            match kind {
                TestKind::Item => "item",
                TestKind::Stmt => "stmt",
                TestKind::Expr => "expr",
            },
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
            kind,
            pass,
        })
    }

    tests
}

fn tests_from_dir(dir: &Path) -> Result<Tests> {
    let mut tests = Tests::default();
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() || entry.path().extension().unwrap_or_default() != "rs" {
            continue;
        }

        process_file(&mut tests, entry.path())?;
    }

    let total = tests.pass.len() + tests.fail.len();
    eprintln!(
        "found {} inline test{}",
        total,
        if total == 1 { "" } else { "s" },
    );

    Ok(tests)
}

fn process_file(res: &mut Tests, path: &Path) -> Result<()> {
    let text = fs::read_to_string(path)?;

    for test in collect_tests(&text) {
        if test.pass {
            if let Some(old_test) = res.pass.insert(test.name.clone(), test) {
                anyhow::bail!("duplicate test: {}", old_test.name);
            }
        } else if let Some(old_test) = res.fail.insert(test.name.clone(), test) {
            anyhow::bail!("duplicate test: {}", old_test.name);
        }
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
            // TODO: Allow specifying kind within tests
            kind: TestKind::Item,
            pass: ok,
        };

        if let Some(old) = res.insert(name, (path, test)) {
            eprintln!("duplicate test: {:?}", old);
        }
    }

    Ok(res)
}
