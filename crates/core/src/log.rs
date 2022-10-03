use tracing_subscriber::{fmt, prelude::*, EnvFilter, layer::Filter};
use tracing_chrome::{FlushGuard, ChromeLayer, ChromeLayerBuilder};
pub fn init_tracing() -> FlushGuard{
  let (chrome_layer,guard) = ChromeLayerBuilder::new().build();
  tracing_subscriber::registry().with(fmt::layer().pretty().with_file(false))
  .with(chrome_layer)
  .with(EnvFilter::from_env("TRACE"))
  
  .init();
  tracing::trace!("enable tracing");
  guard
}