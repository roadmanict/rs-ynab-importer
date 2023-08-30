use clap::Parser;
use std::{error::Error, fs};

use camt053_parser::Camt053Parser;
use ynab_csv::YnabCsvSerializer;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let camt053_parser = Camt053Parser::create();
    let ynab_csv_serializer = YnabCsvSerializer::create();
    let xml = fs::read_to_string(&args.file)?;
    let entries = camt053_parser.parse_file(&xml)?;
    let ynab_csv = ynab_csv_serializer.serialize(entries)?;

    println!("{}", ynab_csv);

    Ok(())
}
