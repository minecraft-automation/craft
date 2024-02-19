use std::num::NonZeroUsize;

use cli::Args;

use eyre::Context;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use tokio::runtime::Builder;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    install_tracing()?;

    let args = Args::from_argv();
    let runtime = match args.worker_threads.map_or(0, NonZeroUsize::get) {
        0 | 1 => Builder::new_current_thread(),
        n => {
            let mut b = Builder::new_multi_thread();
            b.worker_threads(n);
            b
        }
    }
    .thread_name("craft-worker")
    .enable_all()
    .build()
    .wrap_err("failed to create asynchronous runtime!")?;

    runtime.block_on(entrypoint::dispatch_args(args))
}

fn install_tracing() -> eyre::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .without_time()
        .with_ansi(cfg!(not(windows)))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .wrap_err("failed to set global tracing subscriber")?;

    Ok(())
}

mod cli;
mod entrypoint;
