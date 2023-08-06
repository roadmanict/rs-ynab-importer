use std::{
    io::{Error, ErrorKind},
    rc::Rc,
};

use thiserror::Error;

use serde::Serialize;

use common::Entry;
use output_tracker::{OutputListener, OutputTracker};

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct YnabCsv {
    date: String,
    payee: Option<String>,
    memo: Option<String>,
    outflow: Option<String>,
    inflow: Option<String>,
}

impl YnabCsv {
    pub fn new(
        date: String,
        payee: Option<String>,
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
            Some(value.payee),
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
    output_listener: OutputListener<Vec<YnabCsv>>,
    csv_serializer: Box<dyn CsvSerializer>,
}

impl<'a> YnabCsvSerializer {
    pub fn create_nullable() -> YnabCsvSerializer {
        YnabCsvSerializer {
            output_listener: OutputListener::new(),
            csv_serializer: Box::new(StubbedCsvSerializer {}),
        }
    }

    pub fn create() -> YnabCsvSerializer {
        YnabCsvSerializer {
            output_listener: OutputListener::new(),
            csv_serializer: Box::new(RealCsvSerializer {}),
        }
    }

    pub fn track_output(&mut self) -> Rc<OutputTracker<Vec<YnabCsv>>> {
        self.output_listener.create_tracker()
    }

    pub fn serialize(&self, entries: Vec<Entry>) -> Result<String, SerializeStatementsError> {
        let mut ynab_csv: Vec<YnabCsv> = vec![];
        for stmt in entries {
            ynab_csv.push(stmt.into());
        }

        self.output_listener.track(&ynab_csv);

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
        Ok("asdf".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_statements_test() {
        let mut ynab_csv_serializer = YnabCsvSerializer::create_nullable();
        let tracker = ynab_csv_serializer.track_output();

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

        let mut output = tracker.flush();

        assert_eq!(
            output.remove(0),
            vec![YnabCsv::new(
                "17-12-1999".to_string(),
                Some("Albert Heijn".to_string()),
                Some("Memo".to_string()),
                None,
                Some("120".to_string()),
            )]
        );

        assert_eq!(result, "asdf");
    }
}
