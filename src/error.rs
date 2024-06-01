// error.rs
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error")]
    Io(#[from] io::Error),
    #[error("Other Error: {0}")]
    Other(String),
}
