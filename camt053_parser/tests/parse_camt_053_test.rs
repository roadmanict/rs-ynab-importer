use std::fs;

use camt053_parser::Camt053Parser;
use common::Entry;

#[test]
fn name() {
    let camt053_parser = Camt053Parser::create();
    let file_contents = fs::read_to_string("resources/example.xml").expect("File to be read");

    let mut result = camt053_parser.parse_file(&file_contents).expect("xml file to be parsed");

    assert_eq!(result.remove(0), Entry::new("".to_string(), "".to_string(), None, None, None, None));
}
