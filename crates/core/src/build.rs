use anyhow::{Result};
use crate::{Compilation,Compiler, CompilerOptions};
#[derive(Debug)]
pub struct BuildOptions {
  pub context: String,
  pub config: String,
}
pub fn build(_options: BuildOptions) -> Result<()>{
  tracing::debug!("build options:{:?}", _options);
  let compiler = Compiler::new(CompilerOptions {  });
  compiler.compile();
  Ok(())
}