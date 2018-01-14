use decimal::d128;

use ofx::Ofx;

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

pub trait Bank: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn accounts(&self) -> Result<Vec<BankAccount>, String>;
    fn amount(&self, account: &BankAccount) -> Result<d128, String>;
    fn as_ofx<'a>(&'a self) -> Option<&'a Ofx> { None }
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
