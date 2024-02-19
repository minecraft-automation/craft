use std::path::PathBuf;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
/// Manage known minecraft instances
pub enum InstancesSubcommand {
    /// Automatically detect minecraft instances.
    /// 
    /// Can detect instances of: TLauncher, MultiMc (PolyMc, PollyMc, UltiMC, ...)
    Detect,

    /// Adds instance
    Add {
        /// Path to the minecraft folder (for example, /home/<username>/.minecraft)
        path: PathBuf,
    },

    /// Remove instance from known instances
    Forget {
        path: PathBuf,
    }
}
