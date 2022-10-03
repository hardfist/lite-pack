use thiserror::Error;

#[derive(Error, Debug)]
#[error("some io error happened, {:?}", .source)]
pub enum CLIError {
  #[error("IO error: {0}")]
  IOError(#[from] std::io::Error),
  
}