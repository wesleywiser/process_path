//! A Rust library to get the path of the currently executing process.
//! # Example
//! ```
//! let path = process_path::get_executable_path();
//! match path {
//!     None => println!("The process path could not be determined"),
//!     Some(path) => println!("{:?}", path)
//! }
//! ```

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

#[cfg(any(target_os="freebsd", target_os="dragonfly", target_os="netbsd"))]
mod bsd;
#[cfg(any(target_os="freebsd", target_os="dragonfly", target_os="netbsd"))]
use bsd as os;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as os;


/// Gets the path of the currently running process. If the path cannot be determined,
/// `None` is returned.
pub fn get_executable_path() -> Option<PathBuf> {
    os::get_executable_path()
}
