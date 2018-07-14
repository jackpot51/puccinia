use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct CryptoDailyPoint {
    #[serde(rename = "1a. open (USD)")]
    pub open: String,
    #[serde(rename = "2a. high (USD)")]
    pub high: String,
    #[serde(rename = "3a. low (USD)")]
    pub low: String,
    #[serde(rename = "4a. close (USD)")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
    #[serde(rename = "6. market cap (USD)")]
    pub market_cap: String,
}

#[derive(Debug, Deserialize)]
pub struct CryptoDaily {
    #[serde(rename = "Time Series (Digital Currency Daily)")]
    pub series: BTreeMap<String, CryptoDailyPoint>
}
