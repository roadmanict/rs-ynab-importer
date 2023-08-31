use clap::Parser;
use home::home_dir;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, path::Path};

use camt053_parser::Camt053Parser;
use ynab_csv::YnabCsvSerializer;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    file: String,
    #[arg(short, long, default_value_t = String::from(".config/transaction-parser"))]
    config_dir: String,
    #[arg(short, long, default_value_t = false)]
    show_empty_payee: bool,
    #[arg(short, long)]
    account: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    account_alias: HashMap<String, String>,
    payee_regex: HashMap<String, Vec<String>>,
}

struct PayeeRegex {
    payee: String,
    regex: Regex,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let home_dir = home_dir().expect("Home dir to be found.");
    let config_path = Path::new(&home_dir)
        .join(&args.config_dir)
        .join("config.yaml");
    let config_yaml = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config_yaml)?;

    let account = config
        .account_alias
        .get(&args.account)
        .unwrap_or(&args.account);

    let mut payee_regex_list: Vec<PayeeRegex> = vec![];
    for (key, list) in config.payee_regex.into_iter() {
        for r in list.iter() {
            payee_regex_list.push(PayeeRegex {
                payee: key.to_owned(),
                regex: Regex::new(r)?,
            });
        }
    }

    let camt053_parser = Camt053Parser::create();
    let ynab_csv_serializer = YnabCsvSerializer::create();
    let xml = fs::read_to_string(&args.file)?;
    let entries = camt053_parser.parse_file(&xml)?;

    let mut entries = entries
        .into_iter()
        .filter(|e| e.account.eq(account))
        .collect::<Vec<_>>();

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
