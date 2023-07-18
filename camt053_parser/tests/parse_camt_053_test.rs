use camt053_parser::parse_file;

#[test]
fn name() {
    parse_file("resources/example.xml").expect("xml file to be parsed");
}
