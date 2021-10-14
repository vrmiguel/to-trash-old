use std::{ffi::CString, mem, time::Duration};

use libc::{c_char, localtime_r, size_t, time, tm};
use unixstring::UnixString;

use crate::error::Result;

const BUF_SIZ: usize = 64;

extern "C" {
    pub fn strftime(
        s: *mut c_char,
        maxsize: size_t,
        format: *const c_char,
        timeptr: *const tm,
    ) -> size_t;

    pub fn tzset();
}

pub fn format_time(now: Duration) -> Result<String> {
    let mut timestamp = now.as_secs();

    // Safety: the all-zero byte-pattern is valid struct tm
    let mut new_time: tm = unsafe { mem::zeroed() };

    // Safety: time is memory-safe
    // TODO: it'd be better to call `time(NULL)` here
    let ltime = unsafe { time(&mut timestamp as *mut u64 as *mut i64) };

    unsafe { tzset() };

    // Safety: localtime_r is memory safe, threadsafe.
    unsafe { localtime_r(&ltime as *const i64, &mut new_time as *mut tm) };

    let mut char_buf: [c_char; BUF_SIZ] = [0; BUF_SIZ];

    // RFC3339 timestamp
    // Safety: this unwrap is safe since CString::new only fails when
    // the given string has an interior nul char.
    let format = CString::new("%Y-%m-%dT%T").unwrap();

    unsafe {
        strftime(
            char_buf.as_mut_ptr(),
            BUF_SIZ,
            format.as_ptr(),
            &new_time as *const tm,
        )
    };

    let unx = unsafe { UnixString::from_ptr(char_buf.as_ptr()) };

    Ok(unx.to_string_lossy().into())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use chrono::Local;

    use crate::ffi;

    #[test]
    fn rfc3339_formatting() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("it seems that time went backwards!");

        // We'll use the chrono crate to make sure that
        // our own formatting (done through libc's strftime) works
        let date_time = Local::now();

        // YYYY-MM-DDThh:mm:ss
        let rfc3339 = date_time.format("%Y-%m-%dT%T").to_string();

        assert_eq!(&rfc3339, &ffi::format_time(now).unwrap());
    }
}
