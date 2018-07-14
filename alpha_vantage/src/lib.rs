extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::sync::Mutex;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub use daily::{Daily, DailyPoint};
pub use daily_adjusted::{DailyAdjusted, DailyAdjustedPoint};
pub use weekly::{Weekly, WeeklyPoint};
pub use weekly_adjusted::{WeeklyAdjusted, WeeklyAdjustedPoint};
pub use monthly::{Monthly, MonthlyPoint};
pub use monthly_adjusted::{MonthlyAdjusted, MonthlyAdjustedPoint};
pub use crypto_daily::{CryptoDaily, CryptoDailyPoint};

mod daily;
mod daily_adjusted;
mod weekly;
mod weekly_adjusted;
mod monthly;
mod monthly_adjusted;
mod crypto_daily;

// Helper function for errors
pub (crate) fn err_str<E: ::std::fmt::Display>(err: E) -> String {
    format!("{}", err)
}

pub struct AlphaVantage {
    apikey: String,
    query_interval: Duration,
    query_time: Mutex<Option<Instant>>,
}

impl AlphaVantage {
    pub fn new(apikey: &str) -> Self {
        Self {
            apikey: apikey.to_string(),
            query_interval: Duration::new(2, 0),
            query_time: Mutex::new(None),
        }
    }

    fn query_wait(&self) {
        for _i in 0..10 {
            let wait_time_opt = {
                let query_time_opt = self.query_time.lock().unwrap();
                if let Some(query_time) = *query_time_opt {
                    let elapsed = query_time.elapsed();
                    if elapsed < self.query_interval {
                        Some(self.query_interval - elapsed)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some(wait_time) = wait_time_opt {
                sleep(wait_time);
            } else {
                break;
            }
        }

        {
            let mut query_time_opt = self.query_time.lock().unwrap();
            *query_time_opt = Some(Instant::now());
        }
    }

    pub fn query<'a>(&'a self, function: &'a str) -> QueryBuilder<'a> {
        self.query_wait();

        QueryBuilder {
            apikey: &self.apikey,
            function: function,
            query: String::new()
        }
    }

    pub fn daily(&self, symbol: &str, full: bool) -> Result<Daily, String> {
        let mut query = self.query("TIME_SERIES_DAILY")
            .param("symbol", symbol);

        if full {
            query = query.param("outputsize", "full");
        }

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }


    pub fn daily_adjusted(&self, symbol: &str, full: bool) -> Result<DailyAdjusted, String> {
        let mut query = self.query("TIME_SERIES_DAILY_ADJUSTED")
            .param("symbol", symbol);

        if full {
            query = query.param("outputsize", "full");
        }

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }

    pub fn weekly(&self, symbol: &str, full: bool) -> Result<Weekly, String> {
        let mut query = self.query("TIME_SERIES_WEEKLY")
            .param("symbol", symbol);

        if full {
            query = query.param("outputsize", "full");
        }

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }


    pub fn weekly_adjusted(&self, symbol: &str, full: bool) -> Result<WeeklyAdjusted, String> {
        let mut query = self.query("TIME_SERIES_WEEKLY_ADJUSTED")
            .param("symbol", symbol);

        if full {
            query = query.param("outputsize", "full");
        }

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }

    pub fn monthly(&self, symbol: &str, full: bool) -> Result<Monthly, String> {
        let mut query = self.query("TIME_SERIES_MONTHLY")
            .param("symbol", symbol);

        if full {
            query = query.param("outputsize", "full");
        }

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }


    pub fn monthly_adjusted(&self, symbol: &str, full: bool) -> Result<MonthlyAdjusted, String> {
        let mut query = self.query("TIME_SERIES_MONTHLY_ADJUSTED")
            .param("symbol", symbol);

        if full {
            query = query.param("outputsize", "full");
        }

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }

    pub fn crypto_daily(&self, symbol: &str) -> Result<CryptoDaily, String> {
        let query = self.query("DIGITAL_CURRENCY_DAILY")
            .param("symbol", symbol)
            .param("market", "USD");

        let json = query.build().json().map_err(err_str)?;

        serde_json::from_str(&json).map_err(|err| {
            format!("{}: {}", err, json)
        })
    }
}

pub struct QueryBuilder<'a> {
    apikey: &'a str,
    function: &'a str,
    query: String,
}

impl<'a> QueryBuilder<'a> {
    pub fn param(mut self, key: &str, value: &str) -> Self {
        self.query.push('&');
        self.query.push_str(key);
        self.query.push('=');
        self.query.push_str(value);
        self
    }

    pub fn build(self) -> Query {
        Query {
            url: format!(
                "https://www.alphavantage.co/query?function={}{}&apikey={}",
                self.function,
                self.query,
                self.apikey
            )
        }
    }
}

pub struct Query {
    url: String,
}

impl Query {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn json(&self) -> reqwest::Result<String> {
        reqwest::get(&self.url)?
            .error_for_status()?
            .text()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_stock_quotes() {
        let av = AlphaVantage::new("demo");
        let query = av.query("BATCH_STOCK_QUOTES")
            .param("symbols", "MSFT,FB,AAPL")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=BATCH_STOCK_QUOTES&symbols=MSFT,FB,AAPL&apikey=demo");
        query.json().unwrap();
    }

    #[test]
    fn daily() {
        let av = AlphaVantage::new("demo");

        let query = av.query("TIME_SERIES_DAILY")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=MSFT&apikey=demo");
        query.json().unwrap();

        av.daily("MSFT", false).unwrap();
    }

    #[test]
    fn daily_adjusted() {
        let av = AlphaVantage::new("demo");

        let query = av.query("TIME_SERIES_DAILY_ADJUSTED")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY_ADJUSTED&symbol=MSFT&apikey=demo");
        query.json().unwrap();

        av.daily_adjusted("MSFT", false).unwrap();
    }

    #[test]
    fn weekly() {
        let av = AlphaVantage::new("demo");

        let query = av.query("TIME_SERIES_WEEKLY")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_WEEKLY&symbol=MSFT&apikey=demo");
        query.json().unwrap();

        av.weekly("MSFT", false).unwrap();
    }

    #[test]
    fn weekly_adjusted() {
        let av = AlphaVantage::new("demo");

        let query = av.query("TIME_SERIES_WEEKLY_ADJUSTED")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_WEEKLY_ADJUSTED&symbol=MSFT&apikey=demo");
        query.json().unwrap();

        av.weekly_adjusted("MSFT", false).unwrap();
    }

    #[test]
    fn monthly() {
        let av = AlphaVantage::new("demo");

        let query = av.query("TIME_SERIES_MONTHLY")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_MONTHLY&symbol=MSFT&apikey=demo");
        query.json().unwrap();

        av.monthly("MSFT", false).unwrap();
    }

    #[test]
    fn monthly_adjusted() {
        let av = AlphaVantage::new("demo");

        let query = av.query("TIME_SERIES_MONTHLY_ADJUSTED")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_MONTHLY_ADJUSTED&symbol=MSFT&apikey=demo");
        query.json().unwrap();

        av.monthly_adjusted("MSFT", false).unwrap();
    }
}
