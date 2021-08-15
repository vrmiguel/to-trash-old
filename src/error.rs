use std::{path::PathBuf, str::Utf8Error};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Failed to obtain filename for {0}")]
    FailedToObtainFileName(PathBuf),
    #[error("Failed to convert bytes into a String")]
    StringFromBytesError,
    #[error("UTF8 error: {0}")]
    Utf8(#[from] Utf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;
