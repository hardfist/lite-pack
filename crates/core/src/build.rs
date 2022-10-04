use anyhow::{Result};
#[derive(Debug)]
pub struct BuildOptions {
  pub context: String,
  pub config: String,
}
pub fn build(_options: BuildOptions) -> Result<()>{
  tracing::debug!("build options:{:?}", _options);
  Ok(())
}