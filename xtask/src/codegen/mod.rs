mod parse_tests;
mod syntax_kind;

use crate::utils::CodegenMode;
use anyhow::Result;

pub fn codegen(mode: CodegenMode) -> Result<()> {
    syntax_kind::syntax_kind(mode)?;
    parse_tests::parse_tests(mode)?;

    Ok(())
}
