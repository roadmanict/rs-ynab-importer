use std::io::{Error, ErrorKind};

use thiserror::Error;

use serde::Serialize;

use common::Entry;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct YnabCsv {
    date: String,
    payee: String,
    memo: Option<String>,
    outflow: Option<String>,
    inflow: Option<String>,
}

impl YnabCsv {
    pub fn new(
        date: String,
        payee: String,
        memo: Option<String>,
        outflow: Option<String>,
        inflow: Option<String>,
    ) -> YnabCsv {
        YnabCsv {
            date,
            payee,
            memo,
            outflow,
            inflow,
        }
    }
}

impl From<Entry> for YnabCsv {
    fn from(value: Entry) -> Self {
        YnabCsv::new(
            value.date,
            value.payee,
            value.memo,
            value.outflow,
            value.inflow,
        )
    }
}

#[derive(Debug, Error)]
pub enum SerializeStatementsError {
    #[error("Error serializing csv")]
    SerializeError(#[from] csv::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Parse into string error")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

pub struct YnabCsvSerializer {
    csv_serializer: Box<dyn CsvSerializer>,
}

impl YnabCsvSerializer {
    pub fn create_nullable() -> YnabCsvSerializer {
        YnabCsvSerializer {
            csv_serializer: Box::new(StubbedCsvSerializer {}),
        }
    }

    pub fn create() -> YnabCsvSerializer {
        YnabCsvSerializer {
            csv_serializer: Box::new(RealCsvSerializer {}),
        }
    }

    pub fn serialize(&self, entries: Vec<Entry>) -> Result<String, SerializeStatementsError> {
        let mut ynab_csv: Vec<YnabCsv> = vec![];
        for stmt in entries {
            ynab_csv.push(stmt.into());
        }

        let result = self.csv_serializer.serialize(ynab_csv)?;

        Ok(result)
    }
}

trait CsvSerializer {
    fn serialize(&self, entries: Vec<YnabCsv>) -> Result<String, SerializeStatementsError>;
}

#[derive(Debug)]
struct RealCsvSerializer {}

impl CsvSerializer for RealCsvSerializer {
    fn serialize(&self, entries: Vec<YnabCsv>) -> Result<String, SerializeStatementsError> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for stmt in entries {
            wtr.serialize(stmt)?;
        }

        let result = wtr
            .into_inner()
            .map_err(|_| Error::new(ErrorKind::Other, "Into Inner error"))?;

        let result = String::from_utf8(result)?;

        Ok(result)
    }
}

#[derive(Debug)]
struct StubbedCsvSerializer {}

impl CsvSerializer for StubbedCsvSerializer {
    fn serialize(&self, entries: Vec<YnabCsv>) -> Result<String, SerializeStatementsError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_statements_test() {
        let ynab_csv_serializer = YnabCsvSerializer::create_nullable();
        let result = ynab_csv_serializer
            .serialize(vec![Entry::new(
                "Account",
                "17-12-1999",
                "Albert Heijn",
                Some("Memo"),
                Some("120"),
                None,
            )])
            .expect("stmt to be serialized");

        assert_eq!(result, "asdf");
    }
}
