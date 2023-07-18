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
struct Id {
    #[serde(rename = "IBAN")]
    iban: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Acct {
    id: Id,
}

#[derive(Debug, Deserialize, PartialEq)]
enum CdtDbtIndValue {
    #[serde(rename = "DBIT")]
    Dbit,
    #[serde(rename = "CRDT")]
    Crdt,
}

#[derive(Debug, Deserialize, PartialEq)]
struct CdtDbtInd {
    #[serde(rename = "$text")]
    content: CdtDbtIndValue
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct BookgDt {
    dt: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Ntry {
    amt: String,
    cdt_dbt_ind: CdtDbtInd,
    bookg_dt: BookgDt,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Stmt {
    acct: Acct,
    ntry: Vec<Ntry>,
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
pub fn parse_file(xml_contents: &str) -> Result<(), ParseCamt053Error> {
    let camt_053: XmlDocument = from_str(xml_contents)?;

    println!("{:?}", camt_053);

    todo!()
}
