use std::{
    fmt::Display,
    fs::{self, DirEntry},
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

    pub fn list_files(&self, dir: &str) -> Result<Vec<String>, ListFilesError> {
        let paths = fs::read_dir(dir)?;
        let mut paths_vector: Vec<String> = vec![];

        let mut dir_entry: DirEntry;
        for path_result in paths {
            dir_entry = path_result?;

            if let Some(str_path) = dir_entry.path().to_str() {
                paths_vector.push(str_path.to_owned());
            }
        }

        Ok(paths_vector)
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
