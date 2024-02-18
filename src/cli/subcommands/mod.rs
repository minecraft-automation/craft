use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    /// Initialize directory structure for `craft`.
    /// Needed only on the first run.
    Init,

    /// Runs garbage collection
    CollectGarbage {
        #[clap(short, long)]
        older_than: String,
    },

    #[clap(subcommand)]
    Config(config::ConfigSubcommand),

    /// Removes everything related to the `craft`
    Nuke,
}

pub mod config;
