extern crate libc;

use std::path::PathBuf;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as os;

pub fn get_executable_path() -> Option<PathBuf> {
    os::get_executable_path()
}
