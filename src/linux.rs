use std::fs::read_link;
use std::path::PathBuf;

#[inline]
pub fn get_executable_path() -> Option<PathBuf> {
    read_link("/proc/self/exe").ok()
}
