use reqwest;
use std::str;

use err_str;

pub struct BlockchainInfoApi;

impl BlockchainInfoApi {
    pub fn api(&self, path: &str) -> Result<String, String> {
        let url = format!("https://blockchain.info/q/{}", path);

        reqwest::get(&url).map_err(err_str)?
            .error_for_status().map_err(err_str)?
            .text().map_err(err_str)
    }

    pub fn address_balance(&self, address: &str) -> Result<String, String> {
        self.api(&format!("addressbalance/{}", address))
    }

    pub fn daily_price(&self) -> Result<String, String> {
        self.api("24hrprice")
    }
}
