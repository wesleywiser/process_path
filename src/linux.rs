use std::ffi::CString;
use std::path::PathBuf;
use libc::readlink;

pub fn get_executable_path() -> Option<PathBuf> {
    fn get_executable_path(len: usize) -> Option<CString> {
        let mut buf = Vec::with_capacity(len);
        let proc_path = CString::new("/proc/self/exe").expect("path is hardcoded so this should never fail");
        
        unsafe {
            let ret = readlink(proc_path.as_ptr(), buf.as_mut_ptr(), len);
            if ret < 0 {
                None
            } else if (ret as usize) < len {
                buf.set_len(ret as usize);
                CString::new(buf.iter().map(|c| *c as u8).collect::<Vec<_>>()).ok()
            } else {
                //the buffer wasn't big enough, try again with a larger buffer
                get_executable_path(len * 2)
            }
        }
    }
    
    get_executable_path(256)
        .and_then(|p| p.into_string().ok())
        .map(|p| p.into())
}