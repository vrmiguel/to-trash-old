use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::{ffi::CStr, mem, ptr};

use libc::{getpwuid_r, passwd};

use super::effective_user_id;

pub fn get_home_dir() -> Option<PathBuf> {
    let mut buf = [0; 2048];
    let mut result = ptr::null_mut();
    let mut passwd: passwd = unsafe { mem::zeroed() };

    let uid = effective_user_id();

    let getpwuid_r_code =
        unsafe { getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(), buf.len(), &mut result) };

    if getpwuid_r_code == 0 && !result.is_null() {
        // If getpwuid_r succeeded, let's get the username from it
        let home_dir = unsafe { CStr::from_ptr(passwd.pw_dir) };
        let home_dir = OsStr::from_bytes(home_dir.to_bytes());
        let home_dir = PathBuf::from(home_dir);

        return Some(home_dir);
    }

    None
}
