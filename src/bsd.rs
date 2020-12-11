use libc::*;
use std::fs::read_link;
use std::path::PathBuf;
use std::ptr;

// http://stackoverflow.com/q/799679
// http://stackoverflow.com/a/1024937

#[cfg(target_os = "netbsd")]
pub fn get_executable_path() -> Option<PathBuf> {
    read_link("/proc/curproc/exe").ok()
}

#[cfg(target_os = "dragonfly")]
pub fn get_executable_path() -> Option<PathBuf> {
    read_link("/proc/curproc/file").ok()
}

#[cfg(target_os = "freebsd")]
pub fn get_executable_path() -> Option<PathBuf> {
    let mib = [CTL_KERN, KERN_PROC, KERN_PROC_PATHNAME, -1];
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    let mut cb = 1024 as size_t;
    let result = unsafe {
        sysctl(
            mib.as_ptr(),
            4,
            buf.as_mut_ptr() as *mut c_void,
            &mut cb as *mut size_t,
            ptr::null(),
            0,
        )
    };

    // FreeBSD without procfs
    if result == 0 {
        // Convert the string allocated on the stack to a Rust string.
        let len = unsafe { strlen(buf.as_ptr() as *const i8) };
        unsafe { buf.set_len(len) };
        match String::from_utf8(buf).ok() {
            Some(path) => {
                let mut pb = PathBuf::new();
                pb.push(path);
                Some(pb)
            }
            None => None,
        }
    } else {
        // FreeBSD with procfs
        read_link("/proc/curproc/file").ok()
    }
}
