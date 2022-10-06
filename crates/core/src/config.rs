use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub context: String,
  pub entry: HashMap<String, String>
}