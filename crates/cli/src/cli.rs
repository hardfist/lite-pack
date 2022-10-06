use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;
use anyhow::{Result, Context};
use core::build::{build};
use core::config::Config;
use quick_js::{Context as JsContext, JsValue};

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
  tracing::debug!("cwd:{:?}", cwd);
  let root = PathBuf::from(&options.root);
  let root = cwd.join(root).canonicalize()?;
  let config = PathBuf::from(&options.config.as_ref().unwrap_or(&DEFAULT_CONFIG.to_string()));
  tracing::debug!("config:{:?}", config);
  let config_path = root.join(config).canonicalize().expect("config normalize failed");
  
  let root = config_path.parent().unwrap();
  let config_content = std::fs::read_to_string(&config_path)?;
  let context = JsContext::new().expect("create context failed");
  let value = context.eval(&format!("var __dirname = \"{}\";{}{}{}",root.display(),"var module = {exports: {}};", config_content,"JSON.stringify(module.exports)"))?;
  let js_string = value.into_string().expect("generate string failed");
  let config = serde_json::from_str::<Config>(&js_string).with_context(|| format!("failed to load config from {}", config_path.display()))?;
  dbg!(&config);
  build(config)?;
  Ok(())
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