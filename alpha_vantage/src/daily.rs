use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct DailyPoint {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct Daily {
    #[serde(rename = "Time Series (Daily)")]
    pub series: BTreeMap<String, DailyPoint>
}
