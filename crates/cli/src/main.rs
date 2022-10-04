mod cli;
use clap::Parser;
use cli::{Cli, run};
use tracing::{instrument};
use core::log::init_tracing;
use color_eyre::eyre::Result;

fn main() -> Result<()>{
  let guard = init_tracing();
  color_eyre::install()?;
  let cli = Cli::parse();
  run(cli).unwrap_or_else(|err| {
    tracing::error!("{}", err.to_string());
    std::process::exit(1);
  });
  if let Some(guard) = guard {
    guard.flush();
  }
  Ok(())
}