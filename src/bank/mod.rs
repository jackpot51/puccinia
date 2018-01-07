use ofx::Ofx;

pub use self::amex::Amex;
pub use self::tangerine::Tangerine;
pub use self::usaa::Usaa;
pub use self::vanguard::Vanguard;

mod amex;
mod tangerine;
mod usaa;
mod vanguard;

#[derive(Deserialize, Serialize)]
pub struct BankConfig {
    pub kind: String,
    pub username: String,
    pub password: String,
}

impl BankConfig {
    pub fn build(self) -> Result<Box<Ofx>, String> {
        match self.kind.as_str() {
            "amex" => Ok(Box::new(Amex::new(self.username, self.password))),
            "tangerine" => Ok(Box::new(Tangerine::new(self.username, self.password))),
            "usaa" => Ok(Box::new(Usaa::new(self.username, self.password))),
            "vanguard" => Ok(Box::new(Vanguard::new(self.username, self.password))),
            other => Err(format!("Unknown bank kind: {}", other))
        }
    }
}
