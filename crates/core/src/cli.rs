use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;
use anyhow::{Result};
use crate::build::{build, buildOptions};
const DEFAULT_CONFIG: &str = "webpack.config.js";
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Args,Debug)]
pub struct RawOptions {
    root: String,
    #[arg(short, long, value_name = "config")]
    config: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Dev(RawOptions),
    Build(RawOptions),
}
pub fn build_handler(options: &RawOptions) -> Result<()>{
  let cwd = std::env::current_dir()?;
  let root = PathBuf::from(&options.root);
  let root = cwd.join(root);
  let config = PathBuf::from(&options.config.as_ref().unwrap_or(&DEFAULT_CONFIG.to_string()));
  let config = cwd.join(config);
  build(buildOptions{
    context: root.to_string_lossy().to_string(),
    config: config.to_string_lossy().to_string()
  })
}
pub fn run(cli:Cli) -> Result<()>{
  match &cli.command {
      Commands::Dev(_options) => {
        build_handler(_options)?;
        tracing::debug!("{:?}", _options);
      },
      Commands::Build(_options) => {
        build_handler(_options)?;
        tracing::debug!("{:?}", _options);
      }
  }
  Ok(())
}