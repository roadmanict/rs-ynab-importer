use std::fs;

use quick_xml::de::from_str;
use thiserror::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
enum BkToCstmrStmtItem {
    Stmt(Stmt),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Stmt {
    Id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct BkToCstmrStmt {
    #[serde(rename = "$value")]
    items: Vec<BkToCstmrStmtItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct XmlDocument {
    bk_to_cstmr_stmt: BkToCstmrStmt,
}

#[derive(Error, Debug)]
pub enum ParseCamt053Error {
    #[error("Error opening file")]
    FileError(#[from] std::io::Error),
    #[error("Error parsing xml")]
    ParseError(#[from] quick_xml::de::DeError),
}

// Account, Date, Payee, Memo, Inflow, Outflow
pub fn parse_file(path: &str) -> Result<(), ParseCamt053Error> {
    let file = fs::read_to_string(path)?;

    let camt_053: XmlDocument = from_str(&file)?;

    println!("{:?}", camt_053);

    todo!()
}
