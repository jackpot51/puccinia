use rust_decimal::Decimal;
use std::str::FromStr;

pub struct Custom {
    name: String,
    amount: Decimal,
}

impl Custom {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }
}

#[derive(Deserialize, Serialize)]
pub struct CustomConfig {
    pub name: String,
    pub amount: String,
}

impl CustomConfig {
    pub fn build(self) -> Result<Custom, String> {
        let amount = Decimal::from_str(&self.amount).map_err(|_err| {
            format!("invalid decimal: {}", self.amount)
        })?;

        Ok(Custom {
            name: self.name,
            amount: amount
        })
    }
}
