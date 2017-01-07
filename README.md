# process_path

A Rust library to get the path of the currently executing process.

## Usage

Add this to your `Cargo.toml`:

    [dependencies]
    process_path = "0.1"

and this to your crate root:

    extern crate process_path;

## Example

This program prints its path to stdout:

    extern crate process_path;

    use process_path::get_executable_path;

    fn main() {
        let path = get_executable_path();
        match path {
            None => println!("The process path could not be determined"),
            Some(path) => println!("{:?}", path)
        }
    }

## Supported Platforms

Platform     | Underlying API
------------ | ---------------------------------------------
Linux        | `readlink(/proc/self/exe)`
FreeBSD      | `sysctl(3)` or `readlink(/proc/curproc/file)`
NetBSD       | `readlink(/proc/curproc/exe)`
DragonflyBSD | `readlink(/proc/curproc/file)`
macOS        | `_NSGetExecutablePath()`
Windows      | `GetModuleFileName()`

## License

Copyright Wesley Wiser and process_path contributors.

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
