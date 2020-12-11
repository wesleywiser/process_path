# process_path
A Rust library to get the path of the currently executing process or
the the current dynamic library.

The latter is particualrly useful for ‘plug-in‘ type dynamic libraries
that need to load resources stored relative to the location of the
library in the file system.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
process_path = "0.1.3"
```
and this to your crate root:
```rust
use process_path;
```
## Example
This program prints its path to stdout:
```rust
use process_path::get_executable_path;

fn main() {
    let path = get_executable_path();
    match path {
        None => println!("The process path could not be determined"),
        Some(path) => println!("{:?}", path)
    }
}
```

## Supported Platforms

Platform     | Underlying API `get_executable_path()`        | `get_dylib_path()`
------------ | --------------------------------------------- | ---------------------
Linux        | `readlink(/proc/self/exe)`                    | `dladdr()`
FreeBSD      | `sysctl(3)` or `readlink(/proc/curproc/file)` | `dladdr()`
NetBSD       | `readlink(/proc/curproc/exe)`                 | `dladdr()`
DragonflyBSD | `readlink(/proc/curproc/file)`                | `dladdr()`
macOS        | `_NSGetExecutablePath()`                      | `dladdr()`
Windows      | `GetModuleFileName()`                         | `GetModuleHandleEx()`


## License
Copyright Wesley Wiser and `process_path` contributors.

Licensed under either of
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
