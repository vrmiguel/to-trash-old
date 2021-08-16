use std::ffi::CString;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use libc::lstat;

use crate::error::{Error, Result};

pub struct Stat {
    inner: libc::stat,
}

impl Stat {
    pub fn lstat(path: &Path) -> Result<Self> {
        Ok(Self {
            inner: _lstat(path)?,
        })
    }

    pub fn blocks(&self) -> i64 {
        self.inner.st_blocks
    }
}

fn _lstat(path: &Path) -> Result<libc::stat> {
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