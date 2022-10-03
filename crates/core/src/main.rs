mod cli;
mod error;
mod log;
use clap::Parser;
use cli::Cli;
use tracing::{info, instrument};

use crate::log::init_tracing;
#[instrument]
fn main(){
  let guard = init_tracing();
  let args = Cli::parse();
  build_handler();
  info!("args: {:?}", args);
  guard.flush();
}
struct buildOptions {
  root:String
}
#[instrument]
fn build_handler(){

}