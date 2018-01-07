use coinnect::coinnect::Coinnect;
use coinnect::gdax::GdaxCreds;
use coinnect::exchange::Exchange::Gdax;
use coinnect::types::Pair::BTC_USD;
use decimal::d128;
use std::str::FromStr;

use crypto::Crypto;

use self::blockchain_info::BlockchainInfoApi;

mod blockchain_info;

pub struct Bitcoin {
    address: String
}

impl Bitcoin {
    pub fn new(address: String) -> Bitcoin {
        Bitcoin {
            address: address
        }
    }
}

impl Crypto for Bitcoin {
    fn name(&self) -> &str {
        "bitcoin"
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn balance(&self) -> Result<d128, String> {
        let api = BlockchainInfoApi;
        let response = api.address_balance(&self.address)?;

        let satoshi = d128::from_str(&response).map_err(|_err| {
            format!("invalid decimal: {}", response)
        })?;

        Ok(satoshi/d128!(100000000))
    }

    fn rate(&self) -> Result<d128, String> {
        let creds = GdaxCreds::new("", "", "", "");

        let mut api = Coinnect::new(Gdax, creds).map_err(|err| {
            format!("{}", err)
        })?;

        let ticker = api.ticker(BTC_USD).map_err(|err| {
            format!("{}", err)
        })?;

        let string = format!("{}", ticker.last_trade_price);
        d128::from_str(&string).map_err(|_err| {
            format!("invalid decimal: {}", string)
        })
    }
}
