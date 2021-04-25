use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use winapi::shared::minwindef::{DWORD, MAX_PATH};
use winapi::um::{
    errhandlingapi::GetLastError,
    libloaderapi::{
       GetModuleFileNameW, GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
        GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
    }
};
use winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER;

pub(crate) fn get_executable_path() -> Option<PathBuf> {
    fn get_executable_path(len: usize) -> Option<PathBuf> {
        let mut buf = Vec::with_capacity(len);
        unsafe {
            let ret = GetModuleFileNameW(ptr::null_mut(), buf.as_mut_ptr(), len as DWORD) as usize;
            if ret == 0 {
                None
            } else if ret < len {
                // Success, we need to trim trailing null bytes from the vec.
                buf.set_len(ret);
                let s = OsString::from_wide(&buf);
                Some(s.into())
            } else {
                // The buffer might not be big enough so we need to check errno.
                let errno = GetLastError();
                if errno == ERROR_INSUFFICIENT_BUFFER {
                    get_executable_path(len * 2)
                } else {
                    None
                }
            }
        }
    }

    get_executable_path(MAX_PATH)
}

pub(crate) fn get_dylib_path() -> Option<PathBuf> {
    fn get_dylib_path(len: usize) -> Option<PathBuf> {
        let mut buf = Vec::with_capacity(len);
        unsafe {
            let mut handle_module = core::ptr::null_mut();
            if GetModuleHandleExW(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS
                    | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                get_dylib_path as *const _,
                &mut handle_module,
            ) == 0 {
                None
            } else {
                let ret =
                    GetModuleFileNameW(handle_module, buf.as_mut_ptr(), len as DWORD) as usize;
                if ret == 0 {
                    None
                } else if ret < len {
                    // Success, we need to trim trailing null bytes from the vec.
                    buf.set_len(ret);
                    let s = OsString::from_wide(&buf);
                    Some(s.into())
                } else {
                    // The buffer might not be big enough so we need to check errno.
                    let errno = GetLastError();
                    if errno == ERROR_INSUFFICIENT_BUFFER {
                        get_dylib_path(len * 2)
                    } else {
                        None
                    }
                }
            }
        }
    }

    get_dylib_path(MAX_PATH)
}
