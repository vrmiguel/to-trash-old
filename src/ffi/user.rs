//pub fn real_user_id() -> u32 {
// Safety: the POSIX Programmer's Manual states that
// getuid will always be successful.
//    unsafe { libc::getuid() }
//}

pub fn effective_user_id() -> u32 {
    // Safety: the POSIX Programmer's Manual states that
    // geteuid will always be successful.
    unsafe { libc::geteuid() }
}
