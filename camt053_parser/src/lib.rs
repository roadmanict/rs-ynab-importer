use quick_xml::de::from_str;
use thiserror::Error;

use serde::Deserialize;

use common::Entry;

#[derive(Debug, Deserialize, PartialEq, Clone)]
enum BkToCstmrStmtItem {
    Stmt(Stmt),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Id {
    #[serde(rename = "IBAN")]
    iban: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
struct Acct {
    id: Id,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
enum CdtDbtIndValue {
    #[serde(rename = "DBIT")]
    Dbit,
    #[serde(rename = "CRDT")]
    Crdt,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct CdtDbtInd {
    #[serde(rename = "$text")]
    content: CdtDbtIndValue,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
struct BookgDt {
    dt: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
struct Ntry {
    amt: String,
    cdt_dbt_ind: CdtDbtInd,
    bookg_dt: BookgDt,
}

impl Ntry {
    pub fn new(amount: &str, credit_debit_indicator: CdtDbtIndValue, date: &str) -> Self {
        Ntry {
            amt: String::from(amount),
            cdt_dbt_ind: CdtDbtInd {
                content: credit_debit_indicator,
            },
            bookg_dt: BookgDt {
                dt: String::from(date),
            },
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
struct Stmt {
    acct: Acct,
    ntry: Vec<Ntry>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct BkToCstmrStmt {
    #[serde(rename = "$value")]
    items: Vec<BkToCstmrStmtItem>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct XmlDocument {
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
                        &account,
                        &item.bookg_dt.dt,
                        "Payee",
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

pub struct Camt053Parser {
    xml_parser: Box<dyn XmlParser>,
}

impl Camt053Parser {
    pub fn create_nullable(xml_document: XmlDocument) -> Self {
        Camt053Parser {
            xml_parser: Box::new(StubbedXmlParser { xml_document }),
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

struct StubbedXmlParser {
    xml_document: XmlDocument,
}

impl XmlParser for StubbedXmlParser {
    fn parse_from_str(&self, _xml_contents: &str) -> Result<XmlDocument, ParseCamt053Error> {
        Ok(self.xml_document.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stubbed_xml_parser() {
        let xml_document = XmlDocument {
            bk_to_cstmr_stmt: BkToCstmrStmt { items: vec![] },
        };

        let camt_053_parser = Camt053Parser::create_nullable(xml_document);

        assert_eq!(
            camt_053_parser
                .parse_file("<xml><is><mocked>")
                .expect("File to be parsed"),
            vec![]
        )
    }

    #[test]
    fn test_xml_document_has_entries() {
        let xml_document = XmlDocument {
            bk_to_cstmr_stmt: BkToCstmrStmt {
                items: vec![BkToCstmrStmtItem::Stmt(Stmt {
                    acct: Acct {
                        id: Id {
                            iban: String::from("Iban1234account"),
                        },
                    },
                    ntry: vec![Ntry::new("100", CdtDbtIndValue::Crdt, "19-12-2023")],
                })],
            },
        };

        let camt_053_parser = Camt053Parser::create_nullable(xml_document);

        assert_eq!(
            camt_053_parser
                .parse_file("<xml><is><mocked>")
                .expect("File to be parsed"),
            vec![Entry::new(
                "Iban1234account",
                "19-12-2023",
                "Payee",
                None,
                None,
                None
            )]
        )
    }
}
