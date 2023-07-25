use std::{io::{Error, ErrorKind}, string::FromUtf8Error};

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

pub fn serialize_statements(statements: Vec<Entry>) -> Result<(), SerializeStatementsError> {
    let mut ynab_csv: Vec<YnabCsv> = vec![];
    for stmt in statements {
        ynab_csv.push(stmt.into());
    }

    let mut wtr = csv::Writer::from_writer(vec![]);
    for stmt in ynab_csv {
        wtr.serialize(stmt)?;
    }

    let result = wtr
        .into_inner()
        .map_err(|_| Error::new(ErrorKind::Other, "Into Inner error"))?;

    println!("{:?}", String::from_utf8(result)?);

    todo!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_statements_test() {
        serialize_statements(vec![Entry::new(
            "Account",
            "17-12-1999",
            "Albert Heijn",
            Some("Memo"),
            Some("120"),
            None,
        )])
        .expect("stmt to be serialized");
    }
}
