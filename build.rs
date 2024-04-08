use anyhow::Error;
use vergen::EmitBuilder;

pub fn main() -> Result<(), Error> {
    EmitBuilder::builder().git_sha(true).emit()?;
    Ok(())
}
