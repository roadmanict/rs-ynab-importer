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
        account: String,
        date: String,
        payee: String,
        memo: Option<String>,
        inflow: Option<String>,
        outflow: Option<String>,
    ) -> Self {
        Entry {
            account,
            date,
            payee,
            memo,
            inflow,
            outflow,
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub entries: Vec<Entry>,
}
