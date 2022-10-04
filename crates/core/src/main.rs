mod cli;
mod error;
mod build;
mod log;
use clap::Parser;
use cli::{Cli, run};
use tracing::{info, instrument};

use crate::log::init_tracing;
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