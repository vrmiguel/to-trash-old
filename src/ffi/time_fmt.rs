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

    pub fn tzset();
}

pub fn str_from_u8(buf: &[u8]) -> Result<&str> {
    let first_nul_idx = buf.iter().position(|&c| c == b'\0').unwrap_or(buf.len());

    let bytes = buf
        .get(0..first_nul_idx)
        .ok_or(Error::StringFromBytes)?;

    Ok(str::from_utf8(bytes)?)
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

    let mut char_buf = [0; BUF_SIZ];

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

    let char_buf: Vec<_> = char_buf.iter().map(|&ch| ch as u8).collect();

    Ok(str_from_u8(&char_buf)?.into())
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
