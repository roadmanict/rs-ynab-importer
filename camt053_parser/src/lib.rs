use quick_xml::de::from_str;
use thiserror::Error;

use serde::Deserialize;

use common::Entry;

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
    content: CdtDbtIndValue,
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

#[derive(Debug)]
pub struct EntriesContainer {
    pub entries: Vec<Entry>,
}

impl From<XmlDocument> for EntriesContainer {
    fn from(value: XmlDocument) -> Self {
        let mut container = EntriesContainer { entries: vec![] };

        let items = value.bk_to_cstmr_stmt.items;

        for item in items {
            if let BkToCstmrStmtItem::Stmt(stmt) = item {
                let account = stmt.acct.id.iban;
                for item in stmt.ntry {
                    container.entries.push(Entry::new(
                        account.to_owned(),
                        item.bookg_dt.dt,
                        String::from("Payee"),
                        None,
                        None,
                        None,
                    ));
                }
            }
        }

        container
    }
}

pub fn parse_file(xml_contents: &str) -> Result<Vec<Entry>, ParseCamt053Error> {
    let camt_053: XmlDocument = from_str(xml_contents)?;
    let container: EntriesContainer = camt_053.into();

    Ok(container.entries)
}

pub struct Camt053Parser {
    xml_parser: Box<dyn XmlParser>,
}

impl Camt053Parser {
    pub fn create_nullable() -> Self {
        Camt053Parser {
            xml_parser: Box::new(StubbedXmlParser {}),
        }
    }

    pub fn create() -> Self {
        Camt053Parser {
            xml_parser: Box::new(RealXmlParser {}),
        }
    }

    pub fn parse_file(&self, xml_contents: &str) -> Result<Vec<Entry>, ParseCamt053Error> {
        let camt_053 = self.xml_parser.parse_from_str(xml_contents)?;

        let container: EntriesContainer = camt_053.into();

        Ok(container.entries)
    }
}

trait XmlParser {
    fn parse_from_str(&self, xml_contents: &str) -> Result<XmlDocument, ParseCamt053Error>;
}

struct RealXmlParser {}

impl XmlParser for RealXmlParser {
    fn parse_from_str(&self, xml_contents: &str) -> Result<XmlDocument, ParseCamt053Error> {
        let xml_document: XmlDocument = from_str(xml_contents)?;

        Ok(xml_document)
    }
}

struct StubbedXmlParser {}

impl XmlParser for StubbedXmlParser {
    fn parse_from_str(&self, xml_contents: &str) -> Result<XmlDocument, ParseCamt053Error> {
        Ok(XmlDocument {
            bk_to_cstmr_stmt: BkToCstmrStmt { items: vec![] },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stubbed_xml_parser() {
        let camt_053_parser = Camt053Parser::create_nullable();

        assert_eq!(
            camt_053_parser.parse_file("").expect("File to be parsed"),
            vec![]
        )
    }
}
