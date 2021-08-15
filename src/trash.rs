use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::move_file::move_file;
use crate::{
    error::{Error, Result},
    info_file,
};

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

/// Sends the file given by `path` to the given trash structure
fn _send_to_trash(path: &Path, trash: &Trash) -> Result<OsString> {
    // TODO: this could be in a separate function
    let file_name = if !path.ends_with("..") || path != Path::new(".") {
        path.file_name().unwrap().to_owned()
    } else {
        let canonicalized = fs::canonicalize(&path)?;
        match canonicalized.file_name() {
            Some(canon_path) => canon_path.to_owned(),
            None => return Err(Error::FailedToObtainFileName(path.into())),
        }
    };

    let file_in_trash = trash.files.join(&file_name);
    if file_in_trash.exists() {
        // According to the trash-spec 1.0 states that, a file in the trash
        // must not be overwritten by a newer file with the same filename.
        // For this reason, we'll make a new unique filename for the file we're deleting.
        let new_file_name = make_unique_file_name(&Path::new(&file_name), &*trash.files);
        let file_path = trash.files.join(&new_file_name);
        move_file(path, &*file_path)?;

        Ok(new_file_name)
    } else {
        let new_path = trash.files.join(&file_name);
        println!(
            "Files: {}, path: {}, new-path: {}",
            trash.files.display(),
            path.display(),
            new_path.display()
        );
        move_file(path, &new_path)?;

        Ok(file_name)
    }
}

/// Sends a file to trash
pub fn send_to_trash(to_be_removed: OsString, trash: &Trash) -> Result<()> {
    let path = fs::canonicalize(to_be_removed)?;

    // if path.starts_with(&*HOME_DIR) {
    //     // TODO: check for preexisting file
    //     _send_to_trash(path.as_ref(), &HOME_TRASH)?;
    // } else {
    //     todo!("check for parent trash dir")
    // }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("it seems that time went backwards!");

    let file_name = _send_to_trash(&path, trash)?;

    info_file::build_info_file(&path, &file_name, &trash, now)?;

    if path.is_dir() {
        // Update directorysizes
    }

    Ok(())
}

/// Calculates real size of file at path, recursive if it's a directory.
///
/// Real size is always multiple of INODE_SIZE, but `fs::metadata()` uses `stat` and
/// returns the apparent size.
///
/// TODO: consider using a crate that already does this calculation.
pub fn get_file_size(path: impl AsRef<Path>) -> io::Result<u64> {
    /// Adjust apparent size to real size.
    /// TODO: grab real INODE size.
    /// TODO2: INODE size may change if we are crossing partitions.
    fn adjust_apparent_size_to_inode_size(size: u64) -> u64 {
        const INODE_SIZE: u64 = 4096;
        let remainder = size % INODE_SIZE;
        if remainder == 0 {
            size
        } else {
            size + INODE_SIZE - remainder
        }
    }

    let mut sum = 0;

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        let size = fs::metadata(path)?.len() as u64;
        sum += adjust_apparent_size_to_inode_size(size);
    }
    Ok(sum)
}
