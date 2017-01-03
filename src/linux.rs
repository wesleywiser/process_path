use std::fs::read_link;
use std::path::PathBuf;

pub fn get_executable_path() -> Option<PathBuf> {
    read_link("/proc/self/exe").ok()
}