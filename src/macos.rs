use libc::c_int;
use std::{ffi::CStr, os::raw::c_char, path::PathBuf};

extern "C" {
    #[link(name = "dyld")]
    fn _NSGetExecutablePath(buf: *mut c_char, bufsize: *mut u32) -> c_int;
}

pub fn get_executable_path() -> Option<PathBuf> {
    fn get_executable_path(len: u32) -> Option<PathBuf> {
        let mut buf = Vec::with_capacity(len as usize);
        let mut len = len;
        unsafe {
            let result = _NSGetExecutablePath(buf.as_mut_ptr(), &mut len);
            if result == 0 {
                buf.set_len(len as usize);
                Some(PathBuf::from(
                    CStr::from_ptr(buf.as_ptr()).to_str().unwrap(),
                ))
            } else if result == -1 {
                // _NSGetExecutablePath sets len to the required size.
                get_executable_path(len)
            } else {
                // According to the docs, the possible return values
                // should be >= -1 so this shouldn't happen.
                None
            }
        }
    }

    get_executable_path(256)
}
