use cli::Args;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::from_argv();
    dbg!(args);

    Ok(())
}

mod cli;
