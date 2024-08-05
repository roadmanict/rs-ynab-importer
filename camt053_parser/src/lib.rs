pub mod model;
use model::CdtDbtIndValue;
use quick_xml::de::from_str;
use thiserror::Error;

use crate::model::{BkToCstmrStmtItem, XmlDocument};
use common::Entry;

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
                    let mut payee = item
                        .ntry_dtls
                        .tx_dtls
                        .rltd_pties
                        .map(|r| r.cdtr.or(r.dbtr).map(|c| c.nm))
                        .flatten();
                    let mut memo = item.addtl_ntry_inf.or(item
                        .ntry_dtls
                        .tx_dtls
                        .rmt_inf
                        .and_then(|r| r.ustrd.get(0).cloned())
                        .map(|s| s.to_owned()));

                    if let Some(txt) = memo.as_ref() {
                        let memo_split = txt.split(">").collect::<Vec<_>>();
                        if memo_split.len() > 1 {
                            payee = Some(memo_split[0].trim().to_owned());
                            memo = Some(memo_split[1].trim().to_owned());
                        }
                    }

                    let mut inflow: Option<String> = None;
                    let mut outflow: Option<String> = None;

                    match item.cdt_dbt_ind.content {
                        CdtDbtIndValue::Dbit => outflow = Some(item.amt),
                        CdtDbtIndValue::Crdt => inflow = Some(item.amt),
                    }

                    container.entries.push(Entry::new(
                        account.to_owned(),
                        item.bookg_dt.dt,
                        payee,
                        memo.map(|s| {
                            s.replace("\n", "")
                                .split(' ')
                                .filter(|s| !s.is_empty())
                                .collect::<Vec<_>>()
                                .join(" ")
                        }),
                        inflow,
                        outflow,
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
    use crate::model::*;

    use super::*;

    #[test]
    fn test_empty_xml_parser() {
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
    fn test_xml_document_has_entries_with_payee() {
        let xml_document = XmlDocument {
            bk_to_cstmr_stmt: BkToCstmrStmt {
                items: vec![BkToCstmrStmtItem::Stmt(Stmt {
                    acct: Acct {
                        id: Id {
                            iban: "Iban1234account".to_string(),
                        },
                    },
                    ntry: vec![Ntry::new(
                        "100",
                        CdtDbtIndValue::Crdt,
                        "19-12-2023",
                        Some("Memo".to_string()),
                        Some("Payee".to_string()),
                    )],
                })],
            },
        };

        let camt_053_parser = Camt053Parser::create_nullable(xml_document);

        assert_eq!(
            camt_053_parser
                .parse_file("<xml><is><mocked>")
                .expect("File to be parsed"),
            vec![Entry::new(
                "Iban1234account".to_string(),
                "19-12-2023".to_string(),
                Some("Payee".to_string()),
                Some("Memo".to_string()),
                Some("100".to_string()),
                None,
            )]
        )
    }

    #[test]
    fn test_xml_document_has_entries_without_payee() {
        let xml_document = XmlDocument {
            bk_to_cstmr_stmt: BkToCstmrStmt {
                items: vec![BkToCstmrStmtItem::Stmt(Stmt {
                    acct: Acct {
                        id: Id {
                            iban: "Iban1234account".to_string(),
                        },
                    },
                    ntry: vec![Ntry::new(
                        "100",
                        CdtDbtIndValue::Crdt,
                        "19-12-2023",
                        Some("Memo".to_string()),
                        None,
                    )],
                })],
            },
        };

        let camt_053_parser = Camt053Parser::create_nullable(xml_document);

        assert_eq!(
            camt_053_parser
                .parse_file("<xml><is><mocked>")
                .expect("File to be parsed"),
            vec![Entry::new(
                "Iban1234account".to_string(),
                "19-12-2023".to_string(),
                None,
                Some("Memo".to_string()),
                Some("100".to_string()),
                None,
            )]
        )
    }

    #[test]
    fn test_xml_document_has_entries_splits_memo_gt() {
        let xml_document = XmlDocument {
            bk_to_cstmr_stmt: BkToCstmrStmt {
                items: vec![BkToCstmrStmtItem::Stmt(Stmt {
                    acct: Acct {
                        id: Id {
                            iban: "Iban1234account".to_string(),
                        },
                    },
                    ntry: vec![Ntry::new(
                        "100",
                        CdtDbtIndValue::Crdt,
                        "19-12-2023",
                        Some("Payee > Memo".to_string()),
                        None,
                    )],
                })],
            },
        };

        let camt_053_parser = Camt053Parser::create_nullable(xml_document);

        assert_eq!(
            camt_053_parser
                .parse_file("<xml><is><mocked>")
                .expect("File to be parsed"),
            vec![Entry::new(
                "Iban1234account".to_string(),
                "19-12-2023".to_string(),
                Some("Payee".to_string()),
                Some("Memo".to_string()),
                Some("100".to_string()),
                None,
            )]
        )
    }
}
