use anyhow::Result;
use clap::Parser;

use crate::{
    build::build,
    check::check,
    command::{Args, Command},
    fetch_header::fetch_header,
    run::run,
    utils::chdir_to_workspace_root,
};

mod build;
mod check;
mod command;
mod fetch_header;
mod run;
mod run_vm;
mod utils;

fn main() -> Result<()> {
    chdir_to_workspace_root()?;

    let args = Args::parse();

    match args.command {
        Command::Build => {
            build()?;
        }
        Command::Run => {
            run()?;
        }
        Command::Check => {
            check()?;
        }
        Command::FetchHeader => {
            fetch_header()?;
        }
    }

    Ok(())
}
