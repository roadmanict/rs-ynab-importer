use std::{
    fmt::Display,
    fs::{self, DirEntry, ReadDir},
    io,
    os::unix::fs::DirEntryExt2,
};

#[derive(Debug)]
pub enum ListFilesError {
    IOError(std::io::Error),
}
impl Display for ListFilesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListFilesError::IOError(io_error) => write!(f, "{}", io_error),
        }
    }
}
impl std::error::Error for ListFilesError {}

impl From<std::io::Error> for ListFilesError {
    fn from(err: std::io::Error) -> Self {
        ListFilesError::IOError(err)
    }
}

pub struct ListFiles {}

impl ListFiles {
    pub fn nullable() -> Self {
        Self {}
    }
    pub fn new() -> Self {
        Self {}
    }

    pub fn list_files(&self, dir: &str) -> Result<Vec<&str>, ListFilesError> {
        Ok(fs::read_dir(dir)?
            .collect::<Result<Vec<DirEntry>, io::Error>>()?
            .iter()
            .map(|entry| entry.path().to_str())
            .flatten()
            .collect::<Vec<&str>>())


    }
}

trait FSDirEntry {
    fn str_path(&self) -> Option<&str>;
}

struct RealFSDirEntry {
    dir_entry: DirEntry,
}
impl RealFSDirEntry {
    pub fn new(dir_entry: DirEntry) -> Self {
        Self { dir_entry }
    }
}
impl FSDirEntry for RealFSDirEntry {
    fn str_path(&self) -> Option<&str> {
        self.dir_entry.path().to_str()
    }
}

trait FSReadDir: Iterator {}
struct RealFSReadDir {}
impl FSReadDir for RealFSReadDir {}
impl Iterator for RealFSReadDir {
    type Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

trait FS {
    fn read_dir(path: &str) -> Result<ReadDir, std::io::Error>;
}

struct RealFS {}
impl FS for RealFS {
    fn read_dir(path: &str) -> Result<ReadDir, std::io::Error> {
        return fs::read_dir(path);
    }
}

struct StubbedFS {}
impl FS for StubbedFS {
    fn read_dir(path: &str) -> Result<ReadDir, std::io::Error> {
        return Ok();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_vec_of_dir_contents() {
        let list_files = ListFiles::nullable();

        let result = list_files.list_files("./a/directory");

        assert_eq!(result.unwrap(), vec![String::new()]);
    }
}
