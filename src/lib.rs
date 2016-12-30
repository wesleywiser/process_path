extern crate kernel32;
extern crate libc;
extern crate winapi;

use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as os;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as os;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as os;

pub fn get_executable_path() -> Option<PathBuf> {
    os::get_executable_path()
}
