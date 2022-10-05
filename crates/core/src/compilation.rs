use std::{sync::Arc, collections::HashMap, hash::Hash};
use tracing::{instrument};
use crate::{compiler::CompilerOptions, EntryItem};

#[derive(Debug)]
pub struct Compilation {
  pub options: Arc<CompilerOptions>,
  pub params: Arc<CompilationParams>,
  pub entries: HashMap<String,EntryItem>
}
#[derive(Debug)]
pub struct CompilationParams {

}
#[derive(Debug)]
struct Assets {

}
#[derive(Debug)]
pub enum ResolveKind {
  Import,
  Require,
  DynamicImport
}
#[derive(Debug)]
pub struct ModuleDependency {
  pub specifier: String,
  pub kind:  ResolveKind
}
#[derive(Debug)]
pub struct Dependency {
  pub importer: Option<String>,
  pub detail: ModuleDependency
}
impl Compilation {
  pub fn new(options: Arc<CompilerOptions>, params: Arc<CompilationParams>) -> Self{
    Self { options: options.clone(), params, entries: options.as_ref().entry.iter().map(|(name,path)| {
      (name.clone(), EntryItem{
        path: path.clone()
      })
    }).collect() }
  }
  #[instrument(name ="compilation:make")]
  pub(crate) fn make(&self){
    let mut module_graph = self.create_module_graph();
  }
  #[instrument(name ="compilation:seal")]
  pub(crate) fn seal(&self){

  }
  pub fn entry_deps(&self) -> HashMap<String,Dependency >{
    self.entries.iter().map(|(name,item)| {
      (name.clone(), Dependency {
        importer: None,
        detail: ModuleDependency { specifier: item.path.clone(), kind: ResolveKind::Import }
      } )
    }).collect()
  }
  #[instrument]
  pub(crate) fn get_assets(&self) {

  }
  #[instrument]
  pub fn create_module_graph(&self){
    
  }

}