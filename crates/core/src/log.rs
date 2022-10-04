use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_chrome::{FlushGuard, ChromeLayerBuilder};
pub fn init_tracing() -> Option<FlushGuard>{
  let is_enable_chrome_tracing:bool = std::env::var("CHROME_TRACE").ok().map_or(false, |_| true);
  
  let tracing = tracing_subscriber::registry().with(fmt::layer().pretty().with_file(false))
  .with(EnvFilter::from_env("TRACE"));
  let mut guard = None;
  if is_enable_chrome_tracing {
    let (chrome_layer,_guard) = ChromeLayerBuilder::new().build();
    tracing.with(chrome_layer).init();
    guard = Some(_guard);
  }else {
    tracing.init();
  }
  tracing::trace!("enable tracing");
  guard
}