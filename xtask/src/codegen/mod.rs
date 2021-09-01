mod parse_tests;
mod syntax_kind;

use crate::utils::CodegenMode;
use anyhow::Result;

pub fn codegen(mode: CodegenMode) -> Result<()> {
    match mode {
        CodegenMode::Run => eprintln!("running code generation..."),
        CodegenMode::Check => eprintln!("checking generated code..."),
    }

    syntax_kind::syntax_kind(mode)?;
    parse_tests::parse_tests(mode)?;

    match mode {
        CodegenMode::Run => eprintln!("finished running code generation"),
        CodegenMode::Check => eprintln!("finished checking generated code"),
    }

    Ok(())
}
