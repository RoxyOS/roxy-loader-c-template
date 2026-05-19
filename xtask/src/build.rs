use std::path::PathBuf;

use anyhow::{Context, Result};
use roxy_loader_utils::build_image::build_image;
use xshell::{Shell, cmd};

use crate::{
    fetch_header::fetch_header,
    utils::{build_output_dir, generated_include_dir, kernel_artifact_path},
};

pub fn build() -> Result<PathBuf> {
    let output = kernel_artifact_path()?;
    let build_dir = build_output_dir()?;
    let include_dir = generated_include_dir()?;
    let object = build_dir.join("kernel.o");

    std::fs::create_dir_all(&build_dir)?;
    fetch_header()?;

    let sh = Shell::new()?;

    cmd!(sh, "x86_64-elf-gcc -ffreestanding -fno-stack-protector -fno-pic -mno-red-zone -Wall -Wextra -I {include_dir} -c kernel/src/main.c -o {object}")
    .run()
    .with_context(|| format!("failed to compile kernel object at {}", object.display()))?;

    cmd!(sh, "x86_64-elf-gcc -nostdlib -static -T kernel/linker.ld -o {output} {object}")
        .run()
        .with_context(|| format!("failed to link kernel artifact at {}", output.display()))?;

    build_image(output)
}
