use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    error::{Error, Result},
    info_file,
};
use crate::{ffi::Lstat, move_file::move_file};

use std::time::Duration;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Trash {
    pub files: PathBuf,
    pub directory_sizes: PathBuf,
    pub info: PathBuf,
}

impl Trash {
    pub fn new(trash_root: &Path) -> Self {
        Self {
            files: trash_root.join("files"),
            directory_sizes: trash_root.join("directorysizes"),
            info: trash_root.join("info"),
        }
    }
}

/// Renames the file given by `path` until a path
/// not contained in `dir` is found.
///
/// Example: if `foo` exists in `dir`, then this function returns `foo-1`
///          if `foo` and `foo-1` exist in `dir`, then this function returns `foo-2`
/// Note: assumes that `path` exists in `dir`!
pub fn make_unique_file_name(path: &Path, dir: &Path) -> OsString {
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

/// Sends the file given by `path` to the given trash structure
/// Assumes that `path` is canonicalized.
///
/// In case of success, returns the name of the trashed file
/// exactly as sent to `TRASH/files`.
fn _send_to_trash(path: &Path, trash: &Trash, deletion_date: Duration) -> Result<OsString> {
    // Note: this only works properly assuming that the given
    //       path is canonicalized!
    let file_name = path
        .file_name()
        .ok_or_else(|| Error::FailedToObtainFileName(path.into()))?;

    // Note:
    //
    // From the FreeDesktop Trash spec 1.0:
    //
    //```
    //   When trashing a file or directory, the implementation
    //   MUST create the corresponding file in $trash/info first
    //```
    // Our implementation respects this by calling `build_info_file` before `move_file`

    // Where the file will be sent to once trashed
    let file_in_trash = trash.files.join(&file_name);
    if file_in_trash.exists() {
        // According to the trash-spec 1.0 states that, a file in the trash
        // must not be overwritten by a newer file with the same filename.
        // For this reason, we'll make a new unique filename for the file we're deleting.
        let new_file_name = make_unique_file_name(file_name.as_ref(), &*trash.files);
        let file_path = trash.files.join(&new_file_name);
        info_file::build_info_file(path, &new_file_name, trash, deletion_date)?;

        move_file(path, &*file_path)?;

        return Ok(new_file_name);
    }

    println!(
        "Files: {}, path: {}, new-path: {}",
        trash.files.display(),
        path.display(),
        file_in_trash.display()
    );

    info_file::build_info_file(path, file_name, trash, deletion_date)?;
    move_file(path, &file_in_trash)?;

    Ok(file_name.into())
}

/// Sends a file to trash
pub fn send_to_trash(to_be_removed: OsString, trash: &Trash) -> Result<()> {
    let path = fs::canonicalize(&to_be_removed)?;

    // let origin_metadata = path.metadata()?;
    // let modified = origin_metadata.modified()?;
    // let accessed = origin_metadata.modified()?;

    // if path.starts_with(&*HOME_DIR) {
    //     // TODO: check for preexisting file
    //     _send_to_trash(path.as_ref(), &HOME_TRASH)?;
    // } else {
    //     todo!("check for parent trash dir")
    // }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("it seems that time went backwards!");

    let _file_name = _send_to_trash(&path, trash, now)?;

    if path.is_dir() {
        // TODO: Update directorysizes
        // update_directory_size_cache()
        let _dir_size = directory_size(&path)?;
    }

    Ok(())
}

// TODO: add a test for this
pub fn directory_size(path: impl AsRef<Path>) -> Result<u64> {
    let path = path.as_ref();

    if !path.is_dir() {
        return Err(Error::NotADirectory(path.to_owned()));
    }

    let dir_size_in_blocks = WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter_map(|e| Lstat::lstat(e.path()).ok())
        .map(|file| file.blocks() as u64)
        .sum();

    Ok(dir_size_in_blocks)
}
