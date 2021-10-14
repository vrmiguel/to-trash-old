use std::{ffi::NulError, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Failed to obtain filename for {0}")]
    FailedToObtainFileName(PathBuf),
    #[error("UTF8 error: {0}")]
    InternalNulByte(#[from] NulError),
    #[error("Failed to obtain mount points")]
    FailedToObtainMountPoints,
    #[error("A directory was expected but {0} isn't one")]
    NotADirectory(PathBuf),
    #[error("UnixString error: {0}")]
    UnixString(#[from] unixstring::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
