use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::{move_file::move_file, HOME_DIR, HOME_TRASH, HOME_TRASH_FILES};

/// Sends the file given by `path` to the user's home trash.
/// Assumes that `path` starts with HOME_DIR
fn send_to_home_trash(path: &Path) -> Result<()> {
    debug_assert!(path.starts_with(&*HOME_DIR));

    let canonicalized: PathBuf;

    let file_name = if !path.ends_with("..") || path != Path::new(".") {
        path.file_name().unwrap()
    } else {
        canonicalized = fs::canonicalize(&path)?;
        match canonicalized.file_name() {
            Some(canon_path) => canon_path,
            None => return Err(Error::FailedToObtainFileName(path.into())),
        }
    };

    let file_in_trash = HOME_TRASH_FILES.join(file_name);
    if file_in_trash.exists() {
        // TODO: find new name for this file
    }

    Ok(())
}

/// Sends a file to trash
pub fn send_to_trash(to_be_removed: OsString) -> Result<()> {
    let path = PathBuf::from(to_be_removed);

    if path.starts_with(&*HOME_DIR) {
        // TODO: check for preexisting file
        move_file(path.as_ref(), &*HOME_TRASH)?;
    } else {
        todo!("check for parent trash dir")
    }

    Ok(())
}
