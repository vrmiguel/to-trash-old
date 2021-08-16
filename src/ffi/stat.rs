use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::os::unix::ffi::OsStrExt;

use libc::lstat;

use crate::error::{Error, Result};

pub fn metadata_full(path: &Path) -> Result<libc::stat> {
    let c_path = CString::new(path.as_os_str().as_bytes())?;
    // The all-zero byte-pattern is a valid `struct stat`
    let mut stat_buf = unsafe { mem::zeroed() };

    if -1 == unsafe { lstat(c_path.as_ptr(), &mut stat_buf) } {
        // TODO: check errno
        Err(Error::StatError)
    } else {
        Ok(stat_buf)
    }
}