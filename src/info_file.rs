use std::ffi::OsStr;
use std::path::{Path, PathBuf};
/// The $trash/info directory contains an “information file” for every file and directory in $trash/files. This file MUST have exactly the same name as the file or directory in $trash/files, plus the extension “.trashinfo”7.
///
/// The format of this file is similar to the format of a desktop entry file, as described in the Desktop Entry Specification . Its first line must be [Trash Info].
///
/// It also must have two lines that are key/value pairs as described in the Desktop Entry Specification:
///
///    * The key “Path” contains the original location of the file/directory, as either an absolute pathname (starting with the slash character “/”) or a relative pathname (starting with any other character). A relative pathname is to be from the directory in which the trash directory resides (for example, from $XDG_DATA_HOME for the “home trash” directory); it MUST not include a “..” directory, and for files not “under” that directory, absolute pathnames must be used. The system SHOULD support absolute pathnames only in the “home trash” directory, not in the directories under $topdir.
///        - The value type for this key is “string”; it SHOULD store the file name as the sequence of bytes produced by the file system, with characters escaped as in URLs (as defined by RFC 2396, section 2).
///    * The key “DeletionDate” contains the date and time when the file/directory was trashed. The date and time are to be in the YYYY-MM-DDThh:mm:ss format (see RFC 3339). The time zone should be the user's (or filesystem's) local time. The value type for this key is “string”.

use std::time::Duration;
use crate::error::{Error, Result};
use crate::ffi;
use crate::trash::Trash;

pub fn make_info_file_path(file_name: &OsStr, trash_info_path: &Path) -> PathBuf {
    let mut file_name = file_name.to_owned();
    file_name.push(".trashinfo");
    
    trash_info_path.join(file_name)
}

pub fn build_info_file(original_path: &str, file_name: &OsStr, trash: &Trash, deletion_date: Duration) -> Result<()> {
    // The date and time are to be in the YYYY-MM-DDThh:mm:ss format.
    // The time zone should be the user's (or filesystem's) local time.
    let rfc3339 = ffi::format_time(deletion_date)?;

    // The info file is to be built in $trash/info
    let info_path = &*trash.info;
    
    // This file MUST have exactly the same name as the file or directory in $trash/files, plus the extension “.trashinfo”.
    let info_file_path = make_info_file_path(file_name, info_path);

    Ok(())
}