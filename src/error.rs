// error.rs
use thiserror::Error;
use std::io;

// TODO: improve this. I need backtrace as well...
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error")]
    Io(#[from] io::Error),
}
