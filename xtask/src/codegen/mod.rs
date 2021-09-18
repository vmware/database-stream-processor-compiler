mod generate_ast;
mod parse_tests;

use crate::utils::{checks, CodegenMode};
use anyhow::{Context, Result};
use ungrammar::Grammar;

const GRAMMAR: &str = include_str!("../../../crates/ddlog-syntax/ddlog.ungram");

pub fn codegen(mode: CodegenMode) -> Result<()> {
    checks::rustfmt_exists()?;

    generate_ast::generate_ast(mode)?;
    parse_tests::parse_tests(mode)?;

    Ok(())
}

fn grammar() -> Result<Grammar> {
    GRAMMAR.parse().context("failed to parse ddlog grammar")
}
