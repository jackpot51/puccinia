use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct WeeklyAdjustedPoint {
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
}

#[derive(Debug, Deserialize)]
pub struct WeeklyAdjusted {
    #[serde(rename = "Weekly Adjusted Time Series")]
    pub series: BTreeMap<String, WeeklyAdjustedPoint>
}
