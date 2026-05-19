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

pub fn chdir_to_workspace_root() -> Result<()> {
    let root = workspace_root()?;
    Shell::new()?.change_dir(root);
    Ok(())
}

pub fn cargo_target_dir() -> Result<PathBuf> {
    Ok(MetadataCommand::new().exec()?.target_directory.into())
}

pub fn build_output_dir() -> Result<PathBuf> {
    Ok(workspace_root()?.join("build"))
}

pub fn generated_include_dir() -> Result<PathBuf> {
    Ok(build_output_dir()?.join("include"))
}

pub fn downloaded_header_path() -> Result<PathBuf> {
    Ok(generated_include_dir()?.join("roxy_loader.h"))
}

pub fn kernel_artifact_path() -> Result<PathBuf> {
    Ok(build_output_dir()?.join("kernel.elf"))
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

fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest_dir
        .parent()
        .ok_or_else(|| anyhow::anyhow!("xtask manifest dir has no parent"))?
        .to_path_buf())
}
