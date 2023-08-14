use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum BkToCstmrStmtItem {
    Stmt(Stmt),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Id {
    #[serde(rename = "IBAN")]
    pub iban: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Acct {
    pub id: Id,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum CdtDbtIndValue {
    #[serde(rename = "DBIT")]
    Dbit,
    #[serde(rename = "CRDT")]
    Crdt,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CdtDbtInd {
    #[serde(rename = "$text")]
    content: CdtDbtIndValue,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BookgDt {
    pub dt: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RmtInf {
    pub ustrd: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TxDtls {
    pub rmt_inf: RmtInf,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NtryDtls {
    pub tx_dtls: TxDtls,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Ntry {
    amt: String,
    cdt_dbt_ind: CdtDbtInd,
    pub bookg_dt: BookgDt,
    pub ntry_dtls: NtryDtls,
}

impl Ntry {
    pub fn new(
        amount: &str,
        credit_debit_indicator: CdtDbtIndValue,
        date: &str,
        memo: String,
    ) -> Self {
        Ntry {
            amt: amount.to_string(),
            cdt_dbt_ind: CdtDbtInd {
                content: credit_debit_indicator,
            },
            bookg_dt: BookgDt {
                dt: date.to_string(),
            },
            ntry_dtls: NtryDtls {
                tx_dtls: TxDtls {
                    rmt_inf: RmtInf { ustrd: Some(memo) },
                },
            },
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stmt {
    pub acct: Acct,
    pub ntry: Vec<Ntry>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct BkToCstmrStmt {
    #[serde(rename = "$value")]
    pub items: Vec<BkToCstmrStmtItem>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct XmlDocument {
    pub bk_to_cstmr_stmt: BkToCstmrStmt,
}
