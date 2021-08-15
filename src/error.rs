use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Failed to obtain filename for {0}")]
    FailedToObtainFileName(PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;
