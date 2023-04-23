use std::{
    fmt::Display,
    fs::{self},
    io,
    path::PathBuf,
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

pub struct ListFiles {
    list_files_wrapper: Box<dyn ListFilesWrapper>,
}

impl ListFiles {
    pub fn nullable() -> Self {
        Self {
            list_files_wrapper: Box::new(StubbedListFiles {}),
        }
    }
    pub fn new() -> Self {
        Self {
            list_files_wrapper: Box::new(RealListFiles {}),
        }
    }

    pub fn list_files(&self, dir: &str) -> Result<Vec<PathBuf>, ListFilesError> {
        self.list_files_wrapper.list_files(dir)
    }
}
impl Default for ListFiles {
    fn default() -> Self {
        Self::new()
    }
}

trait ListFilesWrapper {
    fn list_files(&self, dir: &str) -> Result<Vec<PathBuf>, ListFilesError>;
}

struct RealListFiles {}
impl ListFilesWrapper for RealListFiles {
    fn list_files(&self, dir: &str) -> Result<Vec<PathBuf>, ListFilesError> {
        Ok(fs::read_dir(dir)?
            .collect::<Result<Vec<_>, io::Error>>()?
            .iter()
            .map(|entry| entry.path())
            .collect::<Vec<_>>())
    }
}

struct StubbedListFiles {}
impl ListFilesWrapper for StubbedListFiles {
    fn list_files(&self, _dir: &str) -> Result<Vec<PathBuf>, ListFilesError> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_vec_of_dir_contents() {
        let path = PathBuf::new();
        let list_files = ListFiles::nullable();

        let result = list_files.list_files("./a/directory");

        assert_eq!(result.unwrap(), vec![path]);
    }
}
