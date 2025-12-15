/*
    Create TempFile struct with a method new that takes a file name as input, it must accept both &str and String types.
    Implement the Drop trait for TempFile to delete the file automatically when the struct goes out of scope.
*/

use std::fs::File;
use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    // 1. Define the `new` associated function
    pub fn new<T: AsRef<str>>(file_name: T) -> std::io::Result<Self>
    where
        PathBuf: From<T>,
    {
        let path_buf = PathBuf::from(file_name);
        File::create(&path_buf)?;
        Ok(Self { path: path_buf })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Your code here to delete the file when TempFile is dropped
        //        std::env::temp_dir()
        std::fs::remove_file(self.path.clone());
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
