use anyhow::Result;

use crate::{build::build, run_vm::run_vm};

pub fn run() -> Result<()> {
    let image = build()?;
    run_vm(image)?;

    Ok(())
}
