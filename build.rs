use anyhow::Error;
use vergen::EmitBuilder;

pub fn main() -> Result<(), Error> {
    // NOTE: This will output everything, and requires all features enabled.
    // NOTE: See the EmitBuilder documentation for configuration options.
    EmitBuilder::builder().git_sha(true).emit()?;
    Ok(())
}
