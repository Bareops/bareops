use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum BareopsError {
    #[error("Initialisation: {0}")]
    Init(String),
    #[error("Invalid state: {0}")]
    InvalidState(String), // should be used to indicate an error that should typically not be possible
    #[error("Taskbook parsing: {0}")]
    TaskbookParse(String),
    #[error("Taskbook execution: {0}")]
    TaskbookExecution(String),
    #[error("I/O: {0}")]
    IO(String),
}

impl From<std::io::Error> for BareopsError {
    fn from(err: std::io::Error) -> BareopsError {
        BareopsError::IO(err.to_string())
    }
}
