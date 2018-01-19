use rust_decimal::Decimal;

pub use self::bitcoin::Bitcoin;

mod bitcoin;

pub trait Crypto: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn amount(&self) -> Result<Decimal, String>;
    fn rate(&self) -> Result<Decimal, String>;
}

#[derive(Deserialize, Serialize)]
pub struct CryptoConfig {
    pub kind: String,
    pub name: String,
    pub address: String,
}

impl CryptoConfig {
    pub fn build(self) -> Result<Box<Crypto>, String> {
        match self.kind.as_str() {
            "bitcoin" => Ok(Box::new(Bitcoin::new(self.name, self.address))),
            other => Err(format!("Unknown crypto kind: {}", other))
        }
    }
}
