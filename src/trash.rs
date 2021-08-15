use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::{move_file::move_file, HOME_DIR, HOME_TRASH, HOME_TRASH_FILES};

// Renames the file given by `path` until a path
// not contained in `dir` is found.
//
// Example: if `foo` exists in `dir`, then this function returns `foo-1`
//          if `foo` and `foo-1` exist in `dir`, then this function returns `foo-2`
// Note: assumes that `path` exists in `dir`!
pub fn make_unique_file_name(path: &Path, dir: &Path) -> OsString {
    // let mut file_name = OsString::from(path);
    let file_name = path.as_os_str();

    for i in 1_u64.. {
        let suffix = format!("-{}", i);
        let mut new_file_name = file_name.to_owned();
        new_file_name.push(suffix);
        let path = dir.join(&new_file_name);
        if !path.exists() {
            return new_file_name;
        }
    }
    
    unreachable!("control really shouldn't reach this")
}

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
        // According to the trash-spec 1.0 states that, a file in the trash
        // must not be overwritten by a newer file with the same filename.
        // For this reason, we'll make a new unique filename for the file we're deleting.
        let file_name = make_unique_file_name(&Path::new(file_name), &*HOME_TRASH_FILES);
        let file_name = HOME_TRASH_FILES.join(file_name);
        move_file(path, Path::new(&*file_name))?;
    } else {
        move_file(path, &*HOME_TRASH_FILES)?;
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
