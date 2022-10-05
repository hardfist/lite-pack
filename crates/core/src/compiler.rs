use std::{sync::Arc, collections::HashMap};

use tracing::{instrument};

use crate::compilation::{Compilation, CompilationParams};
#[derive(Debug, Clone)]
pub struct EntryItem {
  pub path: String,
}
#[derive(Debug)]
pub struct CompilerOptions {
  pub entry: HashMap<String,String>
}
#[derive(Debug)]
pub struct Compiler {
  pub options: Arc<CompilerOptions>,
  pub compilation: Compilation,
}

impl Compiler {
  #[instrument]
  pub fn new(options: CompilerOptions)->Self{
    let options = Arc::new(options);
    let params = Compiler::new_compilation_params();
    let compilation = Compiler::create_compilation(options.clone(), params);
    Self{
      options:options.clone(),
      compilation
    }
  }
  pub fn create_compilation(options: Arc<CompilerOptions>, params: CompilationParams) -> Compilation{
    Compilation::new(options.clone(), Arc::new(params))
  }
  pub fn new_compilation_params() -> CompilationParams{
    CompilationParams{}
  }
  pub fn compile(&self){
    let deps = self.compilation.entry_deps();
    dbg!(deps);
    self.compilation.make();
    self.compilation.seal();

  }
  pub fn watch(){
    todo!()
  }
  pub fn emit_assets(&self){
    let assets = self.compilation.get_assets();
    dbg!(assets)
  }

}
