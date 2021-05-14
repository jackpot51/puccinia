use rust_decimal::Decimal;
use std::str::FromStr;

use crypto::Crypto;

use self::blockchain_info::BlockchainInfoApi;

mod blockchain_info;

pub struct Bitcoin {
    name: String,
    address: String
}

impl Bitcoin {
    pub fn new(name: String, address: String) -> Bitcoin {
        Bitcoin {
            name: name,
            address: address
        }
    }
}

impl Crypto for Bitcoin {
    fn kind(&self) -> &str {
        "bitcoin"
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn amount(&self) -> Result<Decimal, String> {
        let api = BlockchainInfoApi;
        let response = api.address_balance(&self.address)?;

        let satoshi = Decimal::from_str(&response).map_err(|err| {
            format!("invalid decimal: {}: {}", response, err)
        })?.normalize();

        let amount = (satoshi/Decimal::new(100000000, 0)).normalize();
        Ok(amount)
    }

    fn rate(&self) -> Result<Decimal, String> {
        let api = BlockchainInfoApi;
        let response = api.daily_price()?;

        let decimal = Decimal::from_str(&response).map_err(|err| {
            format!("invalid decimal: {}: {}", response, err)
        })?;

        Ok(decimal.normalize())
    }
}
