use anyhow::{Result};
use crate::{Compilation,Compiler, CompilerOptions, config::Config};

pub fn build(config: Config) -> Result<()>{
  tracing::debug!("build config:{:?}",config);
  let compiler = Compiler::new(CompilerOptions { 
    entry: config.entry,
   });
  compiler.compile();
  Ok(())
}