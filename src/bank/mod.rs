use chrono::{Date, Utc};
use decimal::d128;
use std::str::FromStr;

use ofx::{Ofx, string_to_date};

pub use self::amex::Amex;
pub use self::tangerine::Tangerine;
pub use self::usaa::Usaa;
pub use self::vanguard::Vanguard;

mod amex;
mod tangerine;
mod usaa;
mod vanguard;

#[derive(Clone, Deserialize, Serialize)]
pub struct BankAccount {
    pub id: String,
    pub kind: String,
}

pub struct BankPosition {
    pub id: String,
    pub name: String,
    pub units: d128,
    pub price: d128,
}

pub struct BankTransaction {
    pub id: String,
    pub name: String,
    pub time: Date<Utc>,
    pub amount: d128,
}

pub struct BankStatement {
    pub positions: Vec<BankPosition>,
    pub transactions: Vec<BankTransaction>,
}

pub trait Bank: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;

    fn as_ofx<'a>(&'a self) -> Option<&'a Ofx> { None }

    fn accounts(&self) -> Result<Vec<BankAccount>, String>;

    fn statement(&self, account: &BankAccount, start: Date<Utc>, end: Date<Utc>) -> Result<BankStatement, String> {
        if let Some(ofx) = self.as_ofx() {
            let response = ofx.ofx(&account.id, &account.kind, Some(start), Some(end))?;

            let mut positions = Vec::new();
            if let Some(balance) = response.balance {
                if let Some(amount) = balance.amount {
                    positions.push(BankPosition {
                        id: "balance".to_string(),
                        name: "Balance".to_string(),
                        units: d128::from_str(&amount).map_err(|_err| {
                            format!("invalid decimal: {}", amount)
                        })?,
                        price: d128!(1)
                    });
                }
            }

            for position in response.positions {
                if let Some(p_id) = position.id {
                    let mut name = None;
                    let mut ticker = None;
                    for security in &response.securities {
                        if let Some(ref s_id) = security.id {
                            if &p_id == s_id {
                                name = security.name.clone();
                                ticker = security.ticker.clone();
                            }
                        }
                    }

                    if let Some(units) = position.units {
                        if let Some(price) = position.unit_price {
                            positions.push(BankPosition {
                                id: ticker.unwrap_or(p_id),
                                name: name.unwrap_or(String::new()),
                                units: d128::from_str(&units).map_err(|()| {
                                    format!("invalid decimal: {}", units)
                                })?,
                                price: d128::from_str(&price).map_err(|()| {
                                    format!("invalid decimal: {}", price)
                                })?,
                            });
                        }
                    }
                }
            }

            let mut transactions = Vec::new();
            for transaction in response.transactions {
                if let Some(id) = transaction.id {
                    if let Some(time) = transaction.time {
                        if let Some(amount) = transaction.amount {
                            transactions.push(BankTransaction {
                                id: id,
                                name: transaction.name.unwrap_or(String::new()),
                                time: string_to_date(&time).map_err(|err| {
                                    format!("invalid date: {}: {}", time, err)
                                })?,
                                amount: d128::from_str(&amount).map_err(|()| {
                                    format!("invalid decimal: {}", amount)
                                })?,
                            });
                        }
                    }
                }
            }

            Ok(BankStatement {
                positions: positions,
                transactions: transactions
            })
        } else {
            Err(format!("Bank::transactions not implemented for {}", self.kind()))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct BankConfig {
    pub kind: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub accounts: Option<Vec<BankAccount>>,
}

impl BankConfig {
    pub fn build(self) -> Result<Box<Bank>, String> {
        match self.kind.as_str() {
            "amex" => Ok(Box::new(Amex::new(self.name, self.username, self.password, self.accounts))),
            "tangerine" => Ok(Box::new(Tangerine::new(self.name, self.username, self.password, self.accounts))),
            "usaa" => Ok(Box::new(Usaa::new(self.name, self.username, self.password, self.accounts))),
            "vanguard" => Ok(Box::new(Vanguard::new(self.name, self.username, self.password, self.accounts))),
            other => Err(format!("Unknown bank kind: {}", other))
        }
    }
}
