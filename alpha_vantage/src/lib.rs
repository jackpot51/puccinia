extern crate reqwest;

pub struct AlphaVantage {
    apikey: String,
}

impl AlphaVantage {
    pub fn new(apikey: &str) -> Self {
        Self {
            apikey: apikey.to_string()
        }
    }

    pub fn query<'a>(&'a self, function: &'a str) -> QueryBuilder<'a> {
        QueryBuilder {
            apikey: &self.apikey,
            function: function,
            query: String::new()
        }
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
        reqwest::get(&self.url)?.error_for_status()?.text()
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
    fn time_series_daily() {
        let av = AlphaVantage::new("demo");
        let query = av.query("TIME_SERIES_DAILY")
            .param("symbol", "MSFT")
            .build();
        assert_eq!(query.url(), "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=MSFT&apikey=demo");
        query.json().unwrap();
    }
}
