use std::path::PathBuf;

use crate::ffi;
use crate::HOME_DIR;

/// Attemps to find the calling user's home directory.
/// Will check for the HOME env. variable first, falling back to
/// checking passwd if HOME isn't set.
pub fn home_dir() -> Option<PathBuf> {
    if let Some(home_dir) = std::env::var_os("HOME") {
        Some(home_dir.into())
    } else {
        ffi::get_home_dir()
    }
}

/// The path of the home trash directory, as specified by FreeDesktop's trash-spec 1.0
/// Ref.: https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html
pub fn home_trash_path() -> PathBuf {
    if let Ok(xdg_home) = std::env::var("XDG_DATA_HOME") {
        let xdg_home = PathBuf::from(xdg_home);
        return xdg_home.join("Trash");
    }

    HOME_DIR.join(".local/share/Trash")
}
