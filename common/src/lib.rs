#[derive(Debug, PartialEq)]
pub struct Entry {
    pub account: String,
    pub date: String,
    pub payee: String,
    pub memo: Option<String>,
    pub inflow: Option<String>,
    pub outflow: Option<String>,
}

impl Entry {
    pub fn new(
        account: &str,
        date: &str,
        payee: &str,
        memo: Option<&str>,
        inflow: Option<&str>,
        outflow: Option<&str>,
    ) -> Self {
        Entry {
            account: String::from(account),
            date: String::from(date),
            payee: String::from(payee),
            memo: memo.map(|m| String::from(m)),
            inflow: inflow.map(|i| String::from(i)),
            outflow: outflow.map(|o| String::from(o)),
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub entries: Vec<Entry>,
}
