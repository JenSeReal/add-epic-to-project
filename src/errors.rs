use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
  #[error("Could not parse operator {0}. Possible values are AND, OR and NOT")]
  CouldNotParseOperator(String),
}
