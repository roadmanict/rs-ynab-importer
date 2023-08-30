use clap::Parser;
use std::{error::Error, fs};

use camt053_parser::Camt053Parser;
use ynab_csv::YnabCsvSerializer;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    file: String,
    #[arg(short, long, default_value_t = false)]
    show_empty_payee: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let camt053_parser = Camt053Parser::create();
    let ynab_csv_serializer = YnabCsvSerializer::create();
    let xml = fs::read_to_string(&args.file)?;
    let mut entries = camt053_parser.parse_file(&xml)?;

    if args.show_empty_payee {
        entries = entries
            .into_iter()
            .filter(|e| e.payee.is_none())
            .collect::<Vec<_>>();
    }

    let ynab_csv = ynab_csv_serializer.serialize(entries)?;

    println!("{}", ynab_csv);

    Ok(())
}
