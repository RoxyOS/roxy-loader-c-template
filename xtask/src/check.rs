use anyhow::Result;
use xshell::{Shell, cmd};

pub fn check() -> Result<()> {
    let sh = Shell::new()?;

    cmd!(sh, "x86_64-elf-gcc --version").run()?;
    cmd!(sh, "qemu-system-x86_64 --version").run()?;
    cmd!(sh, "cargo --version").run()?;

    Ok(())
}
