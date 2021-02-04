use chrono::{Date, Utc};
use rust_decimal::Decimal;
use std::{fs, path};
use std::str::FromStr;

use err_str;
use ofx::{Ofx, Response, string_to_date};

pub use self::amex::Amex;
pub use self::charles_schwab::CharlesSchwab;
pub use self::chase::Chase;
pub use self::fidelity::Fidelity;
pub use self::fidelity_nb::FidelityNb;
pub use self::tangerine::Tangerine;
pub use self::usaa::Usaa;
pub use self::usaa_inv::UsaaInv;
pub use self::vanguard::Vanguard;

mod amex;
mod charles_schwab;
mod chase;
mod fidelity;
mod fidelity_nb;
mod tangerine;
mod usaa;
mod usaa_inv;
mod vanguard;

#[macro_export]
macro_rules! bank_ofx {
    ($kind:literal, $type:ident, {
        $( $body:tt )*
    }) => {
        use $crate::bank::{Bank, BankAccount};
        use $crate::ofx::Ofx;

        pub struct $type {
            name: String,
            username: String,
            password: String,
            bank_id: String,
            client_id: String,
            accounts: Option<Vec<BankAccount>>,
        }

        impl $type {
            pub fn new(
                name: String,
                username: String,
                password: String,
                bank_id: String,
                client_id: String,
                accounts: Option<Vec<BankAccount>>
            ) -> Self {
                Self {
                    name,
                    username,
                    password,
                    bank_id,
                    client_id,
                    accounts,
                }
            }
        }

        impl Bank for $type {
            fn kind(&self) -> &str {
                $kind
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn as_ofx<'a>(&'a self) -> Option<&'a dyn Ofx> {
                Some(self as &dyn Ofx)
            }

            fn accounts(&self) -> Result<Vec<BankAccount>, String> {
                if let Some(ref accounts) = self.accounts {
                    Ok(accounts.clone())
                } else {
                    self.ofx_accounts()
                }
            }
        }

        impl Ofx for $type {
            fn username(&self) -> &str {
                &self.username
            }

            fn password(&self) -> &str {
                &self.password
            }

            fn bank_id(&self) -> &str {
                &self.bank_id
            }

            fn client_id(&self) -> &str {
                &self.client_id
            }

            $( $body )*
        }
    };
}

#[derive(Deserialize, Serialize)]
pub struct BankConfig {
    pub kind: String,
    pub name: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub bank_id: String,
    #[serde(default)]
    pub client_id: String,
    pub accounts: Option<Vec<BankAccount>>,
}

impl BankConfig {
    pub fn build(self) -> Result<Box<dyn Bank>, String> {
        macro_rules! bank_config {
            ($type:ty) => {
                Ok(Box::new(<$type>::new(
                    self.name,
                    self.username,
                    self.password,
                    self.bank_id,
                    self.client_id,
                    self.accounts
                )))
            };
        }

        match self.kind.as_str() {
            "amex" => bank_config!(Amex),
            "charles_schwab" => bank_config!(CharlesSchwab),
            "chase" => bank_config!(Chase),
            "fidelity" => bank_config!(Fidelity),
            "fidelity_nb" => bank_config!(FidelityNb),
            "tangerine" => bank_config!(Tangerine),
            "usaa" => bank_config!(Usaa),
            "usaa_inv" => bank_config!(UsaaInv),
            "vanguard" => bank_config!(Vanguard),
            other => Err(format!("Unknown bank kind: {}", other))
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BankAccount {
    pub id: String,
    pub kind: String,
}

pub struct BankPositionTransaction {
    pub id: String,
    pub name: String,
    pub time: Date<Utc>,
    pub units: Decimal,
    pub price: Decimal,
    pub value: Decimal,
}

pub struct BankPosition {
    pub id: String,
    pub name: String,
    pub units: Decimal,
    pub price: Decimal,
    pub value: Decimal,
    pub transactions: Vec<BankPositionTransaction>
}

pub struct BankTransaction {
    pub id: String,
    pub name: String,
    pub time: Date<Utc>,
    pub amount: Decimal,
}

pub struct BankStatement {
    pub positions: Vec<BankPosition>,
    pub transactions: Vec<BankTransaction>,
}

pub trait Bank: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;

    fn as_ofx<'a>(&'a self) -> Option<&'a dyn Ofx> { None }

    fn accounts(&self) -> Result<Vec<BankAccount>, String>;

    fn statement(&self, account: &BankAccount, start: Option<Date<Utc>>, end: Option<Date<Utc>>) -> Result<BankStatement, String> {
        if let Some(ofx) = self.as_ofx() {
            // Allow override for offline download
            let ofx_file = format!("secret.{}.ofx", account.id);
            let response = if path::Path::new(&ofx_file).exists() {
                println!("Using OFX data from '{}'", ofx_file);
                let response_data = fs::read(&ofx_file).map_err(|err| {
                    format!("failed to read '{}': {}", ofx_file, err)
                })?;
                Response::decode(&response_data).map_err(err_str)?
            } else {
                ofx.ofx(&account.id, &account.kind, start, end)?
            };

            let mut positions = Vec::new();
            if let Some(balance) = response.balance {
                if let Some(amount) = balance.amount {
                    let value = Decimal::from_str(&amount).map_err(|err| {
                        format!("invalid decimal: {}: {}", amount, err)
                    })?;
                    positions.push(BankPosition {
                        id: "balance".to_string(),
                        name: "Balance".to_string(),
                        units: value,
                        price: Decimal::new(1, 0),
                        value: value,
                        transactions: Vec::new()
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

                    let mut transactions = Vec::new();
                    for transaction in &response.position_transactions {
                        if let Some(ref s_id) = transaction.security_id {
                            if &p_id == s_id {
                                if let Some(ref id) = transaction.id {
                                    if let Some(ref settle) = transaction.settle {
                                        if let Some(ref units) = transaction.units {
                                            if let Some(ref price) = transaction.unit_price {
                                                if let Some(ref value) = transaction.total {
                                                    transactions.push(BankPositionTransaction {
                                                        id: id.clone(),
                                                        name: transaction.memo.clone().unwrap_or(String::new()),
                                                        time: string_to_date(&settle).map_err(|err| {
                                                            format!("invalid date: {}: {}", settle, err)
                                                        })?,
                                                        units: Decimal::from_str(&units).map_err(|err| {
                                                            format!("invalid decimal: {}: {}", units, err)
                                                        })?.normalize(),
                                                        price: Decimal::from_str(&price).map_err(|err| {
                                                            format!("invalid decimal: {}: {}", price, err)
                                                        })?.normalize(),
                                                        value: Decimal::from_str(&value).map_err(|err| {
                                                            format!("invalid decimal: {}: {}", value, err)
                                                        })?.normalize(),
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(units) = position.units {
                        if let Some(price) = position.unit_price {
                            if let Some(value) = position.market_value {
                                positions.push(BankPosition {
                                    id: ticker.unwrap_or(p_id),
                                    name: name.unwrap_or(String::new()),
                                    units: Decimal::from_str(&units).map_err(|err| {
                                        format!("invalid decimal: {}: {}", units, err)
                                    })?.normalize(),
                                    price: Decimal::from_str(&price).map_err(|err| {
                                        format!("invalid decimal: {}: {}", price, err)
                                    })?.normalize(),
                                    value: Decimal::from_str(&value).map_err(|err| {
                                        format!("invalid decimal: {}: {}", value, err)
                                    })?.normalize(),
                                    transactions: transactions
                                });
                            }
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
                                amount: Decimal::from_str(&amount).map_err(|err| {
                                    format!("invalid decimal: {}: {}", amount, err)
                                })?.normalize(),
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
