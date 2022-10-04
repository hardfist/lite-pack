mod cli;
use clap::Parser;
use cli::{run, Cli};
use color_eyre::eyre::Result;
use core::log::init_tracing;
use tracing::instrument;

fn main() -> Result<()> {
    let guard = init_tracing();
    color_eyre::install()?;
    let cli = Cli::parse();
    run(cli).unwrap_or_else(|err| {
        tracing::error!("{}", err.to_string());
        if let Some(guard) = guard {
            guard.flush();
        }
        std::process::exit(1);
    });

    Ok(())
}
