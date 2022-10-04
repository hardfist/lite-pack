use std::sync::Arc;
use tracing::{instrument};
use crate::compiler::CompilerOptions;

#[derive(Debug)]
pub struct Compilation {
  pub options: Arc<CompilerOptions>,
  pub params: Arc<CompilationParams>
}
#[derive(Debug)]
pub struct CompilationParams {

}
#[derive(Debug)]
struct Assets {

}
impl Compilation {
  pub fn new(options: Arc<CompilerOptions>, params: Arc<CompilationParams>) -> Self{
    Self { options, params }
  }
  #[instrument]
  pub(crate) fn make(&self){
    
  }
  #[instrument]
  pub(crate) fn seal(&self){

  }
  #[instrument]
  pub(crate) fn get_assets(&self) {

  }
}