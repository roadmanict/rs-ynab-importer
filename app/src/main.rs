use std::{env, error::Error, fs};
use thiserror::Error;

use camt053_parser::Camt053Parser;
use ynab_csv::YnabCsvSerializer;

#[derive(Debug, Error)]
pub enum TransactionImporterError {
    #[error("No file input")]
    NoFileInputError(),
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let args: Vec<String> = env::args().collect();
    let xml_path = current_dir.join(
        args.get(1)
            .ok_or(TransactionImporterError::NoFileInputError())?,
    );

    println!("{:?}", xml_path);

    let camt053_parser = Camt053Parser::create();
    let ynab_csv_serializer = YnabCsvSerializer::create();
    let xml = fs::read_to_string(xml_path)?;
    let entries = camt053_parser.parse_file(&xml)?;
    let ynab_csv = ynab_csv_serializer.serialize(entries)?;

    println!("{}", ynab_csv);

    Ok(())
}
