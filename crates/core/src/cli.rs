use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
  #[arg(short, long, value_name="config")]
  config: Option<PathBuf>,
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand,Debug)]
enum Commands {
  Dev {
    root: String,
  },
  Build {
    root: String,
  }
}