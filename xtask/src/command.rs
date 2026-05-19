#[derive(Debug, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone, Copy)]
pub enum Command {
    #[command(about = "Build the C kernel and bootable disk image")]
    Build,
    #[command(about = "Build and run the C kernel in QEMU")]
    Run,
    #[command(about = "Check host prerequisites")]
    Check,
    #[command(about = "Download roxy_loader.h from GitHub into build/include")]
    FetchHeader,
}
