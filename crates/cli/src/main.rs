mod cli;
use clap::Parser;
use cli::{Cli, run};
use tracing::{instrument};
use core::log::init_tracing;

#[instrument]
fn main(){
  let guard = init_tracing();
  let cli = Cli::parse();
  run(cli).unwrap_or_else(|err| {
    tracing::error!("{}", err.to_string());
    std::process::exit(1);
  });
  if let Some(guard) = guard {
    guard.flush();
  }
}