use std::num::NonZeroUsize;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = "Manages minecraft mods, modpacks, configs",
    long_about = None,
)]
pub struct Args {
    #[clap(long, short)]
    /// Number of workers to use for various things.
    /// Default: number of logical CPU cores
    pub worker_threads: Option<NonZeroUsize>,

    /// Which action perform
    #[clap(subcommand)]
    pub sub: subcommands::Subcommands,
}

impl Args {
    pub fn from_argv() -> Self {
        <Self as Parser>::parse()
    }
}

pub mod subcommands;
