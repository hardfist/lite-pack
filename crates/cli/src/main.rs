mod cli;
use clap::Parser;
use cli::{run, Cli};
use core::log::init_tracing;
use scopeguard::defer;
use anyhow::{Result};
fn main() -> Result<()> {
    let guard = init_tracing();
    defer! {
      if let Some(guard) = guard {
        guard.flush();
       }
    }
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}
