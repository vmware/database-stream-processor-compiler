use anyhow::Result;

mod codegen;
mod utils;

fn main() -> Result<()> {
    codegen::codegen()?;

    Ok(())
}
