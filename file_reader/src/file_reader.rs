use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use self::nullables::*;

pub struct FileReader {
    file: Box<dyn FileOpenWrapper>,
}

impl FileReader {
    pub fn nullable(file_contents: &str) -> FileReader {
        FileReader {
            file: StubbedFileOpen::new(file_contents),
        }
    }

    pub fn new() -> FileReader {
        FileReader {
            file: RealFileOpen::new(),
        }
    }

    pub fn read_file(&self, path: &str) -> Result<String, io::Error> {
        let data_file_path = Path::new(&path);

        let mut data_file = self.file.open(&data_file_path)?;
        let mut content = String::new();
        data_file.read_to_string(&mut content)?;

        Ok(content)
    }
}

mod nullables {
    use super::*;

    pub trait FileReaderWrapper {
        fn read_to_string(&mut self, content: &mut String) -> Result<usize, io::Error>;
    }

    struct RealFileReader {
        file: File,
    }

    impl FileReaderWrapper for RealFileReader {
        fn read_to_string(&mut self, content: &mut String) -> Result<usize, io::Error> {
            self.file.read_to_string(content)
        }
    }

    struct StubbedFileReader {
        file_contents: String,
    }

    impl StubbedFileReader {
        fn new(file_contents: &String) -> StubbedFileReader {
            let contents = file_contents.to_owned();

            StubbedFileReader {
                file_contents: contents,
            }
        }
    }
    impl FileReaderWrapper for StubbedFileReader {
        fn read_to_string(&mut self, content: &mut String) -> Result<usize, io::Error> {
            content.clear();
            content.replace_range(.., &self.file_contents);

            Ok(1)
        }
    }

    pub trait FileOpenWrapper {
        fn open(&self, path: &Path) -> Result<Box<dyn FileReaderWrapper>, io::Error>;
    }

    pub struct RealFileOpen {}

    impl RealFileOpen {
        pub fn new() -> Box<RealFileOpen> {
            Box::new(RealFileOpen {})
        }
    }
    impl FileOpenWrapper for RealFileOpen {
        fn open(&self, path: &Path) -> Result<Box<dyn FileReaderWrapper>, io::Error> {
            let file = File::open(path)?;

            Ok(Box::new(RealFileReader { file }))
        }
    }

    pub struct StubbedFileOpen {
        file_contents: String,
    }

    impl StubbedFileOpen {
        pub fn new(file_contents: &str) -> Box<dyn FileOpenWrapper> {
            let file = file_contents.to_owned();
            Box::new(StubbedFileOpen {
                file_contents: file,
            })
        }
    }

    impl FileOpenWrapper for StubbedFileOpen {
        fn open(&self, _path: &Path) -> Result<Box<dyn FileReaderWrapper>, io::Error> {
            Ok(Box::new(StubbedFileReader::new(&self.file_contents)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reader() {
        let file_reader = FileReader::nullable(&"Test content");

        let result = file_reader.read_file(&"some_path_to_file");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("Test content"));
    }
}
