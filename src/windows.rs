use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use kernel32::{GetLastError, GetModuleFileNameW};
use winapi::minwindef::DWORD;
use winapi::winerror::ERROR_INSUFFICIENT_BUFFER;

pub fn get_executable_path() -> Option<PathBuf> {
    fn get_executable_path(len: usize) -> Option<PathBuf> {
        let mut buf = Vec::with_capacity(len);
        unsafe {
            let ret = GetModuleFileNameW(ptr::null_mut(), buf.as_mut_ptr(), len as DWORD) as usize;
            if ret == 0 {
                None
            } else if ret < len {
                //success, we need to trim trailing null bytes from the vec
                buf.set_len(ret);
                let s = OsString::from_wide(&buf);
                Some(s.into())
            } else {
                //The buffer might not be big enough so we need to check errno
                let errno = GetLastError();
                if errno == ERROR_INSUFFICIENT_BUFFER {
                    get_executable_path(len * 2)
                } else {
                    None
                }
            }
        }
    }

    get_executable_path(256)
}
