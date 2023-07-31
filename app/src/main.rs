use std::error::Error;

use camt053_parser::parse_file;
use ynab_csv::YnabCsvSerializer;

fn main() -> Result<(), Box<dyn Error>> {
    let ynab_csv_serializer = YnabCsvSerializer::create();
    let entries = parse_file("examples/example.xml")?;
    ynab_csv_serializer.serialize(entries)?;

    Ok(())
}
