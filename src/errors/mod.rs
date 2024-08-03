use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum Error {
    #[error("parsing error : `{0}`")]
    Parsing(String),
    #[error("input is empty")]
    Empty,
}
