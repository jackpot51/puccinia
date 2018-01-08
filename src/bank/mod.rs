use decimal::d128;

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

pub trait Bank {
    fn name(&self) -> &str;
    fn accounts(&self) -> Result<Vec<BankAccount>, String>;
    fn amount(&self, account: &BankAccount) -> Result<d128, String>;
}

#[derive(Deserialize, Serialize)]
pub struct BankConfig {
    pub kind: String,
    pub username: String,
    pub password: String,
    pub accounts: Option<Vec<BankAccount>>,
}

impl BankConfig {
    pub fn build(self) -> Result<Box<Bank>, String> {
        match self.kind.as_str() {
            "amex" => Ok(Box::new(Amex::new(self.username, self.password, self.accounts))),
            "tangerine" => Ok(Box::new(Tangerine::new(self.username, self.password, self.accounts))),
            "usaa" => Ok(Box::new(Usaa::new(self.username, self.password, self.accounts))),
            "vanguard" => Ok(Box::new(Vanguard::new(self.username, self.password, self.accounts))),
            other => Err(format!("Unknown bank kind: {}", other))
        }
    }
}
