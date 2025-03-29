use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
