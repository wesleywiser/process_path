use std::{ffi::CStr, os::raw::c_void, path::PathBuf};
use libc;

#[inline]
pub(crate) fn get_dylib_path() -> Option<PathBuf> {
    let mut dl_info = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };

    if unsafe {
        libc::dladdr(
            get_dylib_path as *const c_void,
            &mut dl_info as *mut libc::Dl_info,
        ) != 0
    } {
        if core::ptr::null() == dl_info.dli_fname {
            None
        } else {
            match unsafe { CStr::from_ptr(dl_info.dli_fname) }.to_str() {
                Ok(path) => Some(PathBuf::from(path)),
                Err(_) => None,
            }
        }
    } else {
        None
    }
}
