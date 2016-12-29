use std::ffi::{CString, NulError};
use std::os::raw::c_char;
use std::path::PathBuf;
use libc::{c_int, uint32_t};

extern {
    #[link(name = "dyld")]
    fn _NSGetExecutablePath(buf: *mut c_char, bufsize: *mut uint32_t) -> c_int;
}


pub fn get_executable_path() -> Option<PathBuf> {
    fn get_executable_path(len: uint32_t) -> Result<CString, NulError> {
        let mut buf = Vec::with_capacity(len as usize);
        let mut len = len;
        unsafe {
            let result = _NSGetExecutablePath(buf.as_mut_ptr(), &mut len);
            if result == 0 {
                buf.set_len(len as usize);
                //trim any excess null bytes from the vec
                buf.retain(|c| *c != 0);
                CString::new(buf.iter().map(|c| *c as u8).collect::<Vec<_>>())
            } else if result == -1 {
                get_executable_path(len)
            } else {
                unreachable!();
            }
        }
    }

    get_executable_path(256).ok()
        .and_then(|p| p.into_string().ok())
        .map(|p| p.into())
}
