
use std::{env, io};
use std::fs::{self, File};
use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let mut path_tmp = PathBuf::from(env::temp_dir());

        path_tmp.push(path);

        File::create(&path_tmp)?;
        Ok(Self{
            path: PathBuf::from(&path_tmp)
        })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
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
