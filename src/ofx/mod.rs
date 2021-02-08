use chrono::{Date, TimeZone, Utc};
use reqwest::{Body, Client};
use reqwest::header::{ACCEPT, CONNECTION, CONTENT_TYPE, USER_AGENT};
use std::str;
use std::io::Read;

use err_str;
use bank::BankAccount;

pub use self::request::Request;
pub use self::response::Response;

mod request;
mod response;

pub fn date_to_string(date: &Date<Utc>) -> String {
    date.format("%Y%m%d").to_string()
}

pub fn string_to_date(string: &str) -> Result<Date<Utc>, String> {
    let mut chars = string.chars();

    let year = chars.by_ref().take(4).collect::<String>().parse().map_err(err_str)?;

    let month = chars.by_ref().take(2).collect::<String>().parse().map_err(err_str)?;

    let day = chars.by_ref().take(2).collect::<String>().parse().map_err(err_str)?;

    Ok(Utc.ymd(year, month, day))
}

pub trait Ofx {
    fn url(&self) -> &str;

    fn ofx_ver(&self) -> &str {
        "102"
    }

    fn pretty(&self) -> bool {
        false
    }

    fn user_agent(&self) -> &str {
        "puccinia"
    }

    fn username(&self) -> &str;

    fn password(&self) -> &str;

    fn language(&self) -> &str {
        "ENG"
    }

    fn fid(&self) -> &str;

    fn fid_org(&self) -> &str;

    fn app_id(&self) -> &str {
        "QBKS"
    }

    fn app_ver(&self) -> &str {
        "1900"
    }

    fn client_id(&self) -> &str {
        ""
    }

    fn bank_id(&self) -> &str {
        ""
    }

    fn broker_id(&self) -> &str {
        ""
    }

    fn ofx_accounts(&self) -> Result<Vec<BankAccount>, String> {
        let mut accounts = Vec::new();

        let response = self.ofx("", "", None, None)?;
        for account in response.accounts {
            if let Some(id) = account.id {
                if let Some(kind) = account.kind {
                    accounts.push(BankAccount {
                        id,
                        kind,
                        name: account.description,
                        amount: None,
                    });
                }
            }
        }

        Ok(accounts)
    }

    fn ofx(&self, account_id: &str, account_type: &str, start: Option<Date<Utc>>, end: Option<Date<Utc>>) -> Result<Response, String> {
        let request = Request {
            url: self.url(),
            ofx_ver: self.ofx_ver(),
            pretty: self.pretty(),

            username: self.username(),
            password: self.password(),
            language: self.language(),
            fid: self.fid(),
            fid_org: self.fid_org(),
            app_id: self.app_id(),
            app_ver: self.app_ver(),
            client_id: self.client_id(),

            bank_id: self.bank_id(),
            broker_id: self.broker_id(),
            account_id: account_id,
            account_type: account_type,

            start: start,
            end: end
        };

        let client = Client::builder()
            .cookie_store(true)
            .build().map_err(err_str)?;

        // Vanguard requires one request to get cookies, then the second request will succeed
        let attempts = 2;
        let mut attempt = 1;
        loop {
            println!("Attempt {}/{}", attempt, attempts);

            let request_data = request.encode().map_err(err_str)?;
            println!("Request: [\n{}\n]", str::from_utf8(&request_data).unwrap_or("[Invalid UTF-8]"));

            let mut response = client
                .post(request.url)
                .header(ACCEPT, "application/ofx")
                .header(CONNECTION, "close")
                .header(CONTENT_TYPE, "application/x-ofx")
                .header(USER_AGENT, self.user_agent())
                .body(Body::from(request_data))
                .send().map_err(err_str)?;

            let mut response_data = Vec::new();
            response.read_to_end(&mut response_data).map_err(err_str)?;
            println!("Response: [\n{}\n]", str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]"));

            if response.status().is_success() {
                return Ok(Response::decode(&response_data).map_err(err_str)?);
            } else if attempt >= attempts {
                return Err(format!("Error: {}\n{}", response.status(), str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]")));
            } else {
                println!("Error: {}", response.status());
                attempt += 1;
            }
        }
    }
}
