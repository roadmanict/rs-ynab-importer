use ynab_importer::parse_file;

#[test]
fn name() {
    parse_file("resources/example.xml").expect("xml file to be parsed");
}
