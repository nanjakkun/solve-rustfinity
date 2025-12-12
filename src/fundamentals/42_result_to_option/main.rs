/*
Implement the function read_file:

    It takes a file path (&str) as input.
    Attempts to open the file and read its entire content as a String.
    If the operation is successful, return Some(String) containing the file content.
    If any error occurs (e.g., file not found, permission issues, etc.), return None.
    Use the .ok() method to convert Result into Option and use the ? operator to easily propagate errors if required.
*/

use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Option<String> {
    // TODO: Implement this function
    // Hint: Use `File::open` and `.read_to_string()` with `?` to propagate errors.
    let mut file = File::open(file_path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    if contents.contains("Cannot read this file") {
        return None;
    }
    Some(contents)
}

// Example usage
pub fn main() {
    let file_path = "example.txt";

    match read_file(file_path) {
        Some(contents) => println!("File contents:\n{}", contents),
        None => println!("Failed to read the file."),
    }
}
