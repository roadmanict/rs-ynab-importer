use std::error::Error;

use camt053_parser::parse_file;
use ynab_csv::serialize_statements;

fn main() -> Result<(), Box<dyn Error>> {
    let statements = parse_file("examples/example.xml")?;

    serialize_statements(statements);

    Ok(())
}
