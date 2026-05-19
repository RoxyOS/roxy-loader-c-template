use std::path::PathBuf;

use anyhow::{Context, Result};
use roxy_loader_utils::build_image::build_image;
use xshell::{Shell, cmd};

use crate::{
    fetch_header::fetch_header,
    utils::{build_dir, include_dir, kernel_output_path},
};

pub fn build() -> Result<PathBuf> {
    let output = kernel_output_path()?;
    let build_dir = build_dir()?;
    let include_dir = include_dir()?;

    std::fs::create_dir_all(&build_dir)?;
    fetch_header()?;

    let sh = Shell::new()?;

    cmd!(sh, "clang -target x86_64-unknown-none -ffreestanding -fno-stack-protector -fno-pic -mno-red-zone -Wall -Wextra -I {include_dir} -nostdlib -Wl,-T,kernel/linker.ld -Wl,-static -Wl,--no-pie -fuse-ld=lld kernel/src/main.c -o {output}")
    .run()
    .with_context(|| format!("failed to build kernel artifact at {}", output.display()))?;

    build_image(output)
}
