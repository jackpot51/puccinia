use coinnect::coinnect::Coinnect;
use coinnect::gdax::GdaxCreds;
use coinnect::exchange::Exchange::Gdax;
use coinnect::types::Pair::BTC_USD;
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
        let creds = GdaxCreds::new("", "", "", "");

        let mut api = Coinnect::new(Gdax, creds).map_err(|err| {
            format!("{}", err)
        })?;

        let ticker = api.ticker(BTC_USD).map_err(|err| {
            format!("{}", err)
        })?;

        let string = format!("{}", ticker.last_trade_price);
        let rate = Decimal::from_str(&string).map_err(|err| {
            format!("invalid decimal: {}: {}", string, err)
        })?.normalize();
        Ok(rate)
    }
}
