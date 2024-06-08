// error.rs
use thiserror::Error;
use std::io;

// TODO: Make sure all is covered. backtrace required as well...
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error")]
    Io(#[from] io::Error),
}
