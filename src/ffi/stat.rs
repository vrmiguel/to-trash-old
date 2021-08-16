use std::ffi::CString;
use std::fs::Permissions;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
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

    pub fn mode(&self) -> u32 {
        self.inner.st_mode
    }

    pub fn permissions(&self) -> Permissions {
        Permissions::from_mode(self.mode())
    }

    pub fn blocks(&self) -> i64 {
        self.inner.st_blocks
    }

    pub fn accessed(&self) -> u64 {
        self.inner.st_atime as u64
    }

    pub fn modified(&self) -> u64 {
        self.inner.st_mtime as u64
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

#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;

    use tempfile::NamedTempFile;

    use super::Stat;

    #[test]
    fn time_of_last_modification() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        let mod_timestamp = path
            .metadata()
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let stat = Stat::lstat(path).unwrap();

        assert_eq!(mod_timestamp, stat.modified());
    }
}
