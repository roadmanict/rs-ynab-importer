use std::fs;

use camt053_parser::parse_file;

#[test]
fn name() {
    let file_contents = fs::read_to_string("resources/example.xml").expect("File to be read");

    parse_file(&file_contents).expect("xml file to be parsed");
}
