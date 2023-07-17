use std::io;
use thiserror::Error;

use serde::Serialize;

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

#[derive(Debug, Error)]
pub enum SerializeStatementsError {
    #[error("Error serializing csv")]
    SerializeError(#[from] csv::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

pub fn serialize_statements(statements: Vec<YnabCsv>) -> Result<(), SerializeStatementsError> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for stmt in statements {
        wtr.serialize(stmt)?;
    }

    wtr.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_statements_test() {
        serialize_statements(vec![YnabCsv::new(
            String::from("17-12-1999"),
            String::from("Albert Heijn"),
            Some(String::from("Memo")),
            Some(String::from("120")),
            None,
        )])
        .expect("stmt to be serialized");
    }
}

