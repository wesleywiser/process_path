//! # Executable & Dynamic Library Paths
//! Utility functions to get the path of the currently executing
//! process or the the current dynamic library.
//!
//! The latter is particularly useful for ‘plug-in’ type dynamic
//! libraries that need to load resources stored relative to the
//! location of the library in the file system.
//! ## Example
//! ```
//! let path = process_path::get_executable_path();
//! match path {
//!     None => println!("The process path could not be determined"),
//!     Some(path) => println!("{:?}", path)
//! }
//! ```
//! ## Supported Platforms
//! * Linux
//! * FreeBSD
//! * NetBSD
//! * DragonflyBSD
//! * macOS
//! * Windows
//! * illumos
//! * Android
//! * iOS
use std::path::PathBuf;

#[cfg(any(target_os = "linux", target_os = "illumos", target_os = "android"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "illumos", target_os = "android"))]
use linux as os;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as os;

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "ios")]
use ios as os;

#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd"))]
mod bsd;
#[cfg(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd"))]
use bsd as os;

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "macos",
    target_os = "illumos",
    target_os = "android",
    target_os = "ios",
))]
mod nix;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

/// Gets the path of the currently running process. If the path cannot be determined,
/// `None` is returned.
#[inline]
pub fn get_executable_path() -> Option<PathBuf> {
    os::get_executable_path()
}

/// Gets the path of the current dynamic library. If the path cannot be determined,
/// `None` is returned.
#[inline]
pub fn get_dylib_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        os::get_dylib_path()
    }
    #[cfg(not(target_os = "windows"))]
    {
        nix::get_dylib_path()
    }
}
