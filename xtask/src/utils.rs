use std::{
    env,
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use anyhow::Result;
use cargo_metadata::MetadataCommand;
use qemu_command_builder::{QemuInstanceForX86_64, to_command::ToCommand};
use xshell::Shell;

pub fn chdir_to_repo_root() -> Result<()> {
    let workspace_root = repo_root()?;
    Shell::new()?.change_dir(workspace_root);
    Ok(())
}

pub fn cargo_target_dir() -> Result<PathBuf> {
    Ok(MetadataCommand::new()
        .manifest_path(repo_root()?.join("xtask/Cargo.toml"))
        .exec()?
        .target_directory
        .into())
}

pub fn build_dir() -> Result<PathBuf> {
    Ok(repo_root()?.join("build"))
}

pub fn include_dir() -> Result<PathBuf> {
    Ok(build_dir()?.join("include"))
}

pub fn header_output_path() -> Result<PathBuf> {
    Ok(include_dir()?.join("roxy_loader.h"))
}

pub fn kernel_output_path() -> Result<PathBuf> {
    Ok(build_dir()?.join("kernel.elf"))
}

pub fn run_qemu(qemu_command: QemuInstanceForX86_64) -> Result<ExitStatus> {
    let argv = qemu_command.to_command();
    let [program, args @ ..] = argv.as_slice() else {
        anyhow::bail!("run_qemu: empty qemu command")
    };

    Ok(Command::new(program)
        .args(args)
        .stdout(io::stdout())
        .spawn()?
        .wait()?)
}

fn repo_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("xtask manifest dir has no parent"))?
        .to_path_buf())
}
