#[derive(Debug)]
pub struct Statement {
    account: String,
    date: String,
    payee: String,
    memo: Option<String>,
    inflow: Option<String>,
    outflow: Option<String>,
}
// Account, Date, Payee, Memo, Inflow, Outflow
