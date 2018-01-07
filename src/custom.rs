use decimal::d128;
use std::str::FromStr;

pub struct Custom {
    name: String,
    amount: d128,
}

impl Custom {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn amount(&self) -> d128 {
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
        let amount = d128::from_str(&self.amount).map_err(|_err| {
            format!("invalid decimal: {}", self.amount)
        })?;

        Ok(Custom {
            name: self.name,
            amount: amount
        })
    }
}
