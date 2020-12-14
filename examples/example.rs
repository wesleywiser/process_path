extern crate process_path;

use process_path::get_executable_path;

fn main() {
    let path = get_executable_path();
    match path {
        None => println!("The process path could not be determined"),
        Some(path) => println!("{:?}", path),
    }
}
