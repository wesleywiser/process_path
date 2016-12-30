use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;
use libc::{c_int, uint32_t};

extern {
    #[link(name = "dyld")]
    fn _NSGetExecutablePath(buf: *mut c_char, bufsize: *mut uint32_t) -> c_int;
}


pub fn get_executable_path() -> Option<PathBuf> {
    fn get_executable_path(len: uint32_t) -> Option<CString> {
        let mut buf = Vec::with_capacity(len as usize);
        let mut len = len;
        unsafe {
            let result = _NSGetExecutablePath(buf.as_mut_ptr(), &mut len);
            if result == 0 {
                buf.set_len(len as usize);
                //trim any excess null bytes from the vec
                buf.retain(|c| *c != 0);
                CString::new(buf.iter().map(|c| *c as u8).collect::<Vec<_>>()).ok()
            } else if result == -1 {
                //_NSGetExecutablePath sets len to the required size
                get_executable_path(len)
            } else {
                //according to the docs, the possible return values should be >= -1
                //so this shouldn't happen
                None
            }
        }
    }

    get_executable_path(256)
        .and_then(|p| p.into_string().ok())
        .map(|p| p.into())
}
