use anyhow::{Result};
pub struct buildOptions {
  pub context: String,
  pub config: String,
}
pub fn build(options: buildOptions) -> Result<()>{
  tracing::debug!(options= "options");
  todo!()
}