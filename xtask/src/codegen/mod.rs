mod generate_ast;
mod parse_tests;

use crate::utils::CodegenMode;
use anyhow::{Context, Result};
use ungrammar::Grammar;

pub fn codegen(mode: CodegenMode) -> Result<()> {
    generate_ast::generate_ast(mode)?;
    parse_tests::parse_tests(mode)?;

    Ok(())
}

const GRAMMAR: &str = include_str!("ddlog.ungram");

fn grammar() -> Result<Grammar> {
    GRAMMAR.parse().context("failed to parse ddlog grammar")
}
