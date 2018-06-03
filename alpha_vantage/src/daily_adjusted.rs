use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct DailyAdjustedPoint {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. adjusted close")]
    pub adjusted_close: String,
    #[serde(rename = "6. volume")]
    pub volume: String,
    #[serde(rename = "7. dividend amount")]
    pub dividend_amount: String,
    #[serde(rename = "8. split coefficient")]
    pub split_coefficient: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyAdjusted {
    #[serde(rename = "Time Series (Daily)")]
    pub series: BTreeMap<String, DailyAdjustedPoint>
}
