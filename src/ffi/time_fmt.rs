use std::{ffi::CString, mem, str, time::Duration};

use libc::{c_char, localtime_r, size_t, time, tm};

use crate::error::{Error, Result};

const BUF_SIZ: usize = 64;

extern "C" {
    pub fn strftime(
        s: *mut c_char,
        maxsize: size_t,
        format: *const c_char,
        timeptr: *const tm,
    ) -> size_t;
}

pub fn str_from_u8(buf: &[u8]) -> Result<&str> {
    let first_nul_idx = buf.iter().position(|&c| c == b'\0').unwrap_or(buf.len());

    let bytes = buf
        .get(0..first_nul_idx)
        .ok_or(Error::StringFromBytesError)?;

    Ok(str::from_utf8(bytes)?)
}

pub fn format_time(now: Duration) -> Result<String> {
    let mut timestamp = now.as_secs();

    // Safety: the all-zero byte-pattern is valid struct tm
    let mut new_time: tm = unsafe { mem::zeroed() };

    // Safety: time is memory-safe
    // TODO: it'd be better to call `time(NULL)` here
    let ltime = unsafe { time(&mut timestamp as *mut u64 as *mut i64) };

    // TODO: call tzset before localtime_r ?

    // Safety: localtime_r is memory safe, threadsafe.
    unsafe { localtime_r(&ltime as *const i64, &mut new_time as *mut tm) };

    let mut char_buf = [0; BUF_SIZ];

    // RFC3339 timestamp
    // Safety: this unwrap is safe since CString::new only fails when
    // the given string has an interior nul char.
    let format = CString::new("%FT%T").unwrap();

    unsafe {
        strftime(
            char_buf.as_mut_ptr(),
            BUF_SIZ,
            format.as_ptr(),
            &new_time as *const tm,
        )
    };

    let char_buf: Vec<_> = char_buf.iter().map(|&ch| ch as u8).collect();

    Ok(str_from_u8(&char_buf)?.into())
}
