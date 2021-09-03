use std::{ffi::NulError, path::PathBuf, str::Utf8Error};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Failed to obtain filename for {0}")]
    FailedToObtainFileName(PathBuf),
    #[error("Failed to convert bytes into a String")]
    StringFromBytes,
    #[error("UTF8 error: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("Internal zero byte found during CString construction")]
    InternalNulByte(#[from] NulError),
    #[error("Failed to obtain mount points")]
    FailedToObtainMountPoints,
    #[error("A directory was expected but {0} isn't one")]
    NotADirectory(PathBuf),

    // TODO: check errno when this happens and subdivide the errors
    #[error("stat failed")]
    Stat,
}

pub type Result<T> = std::result::Result<T, Error>;
