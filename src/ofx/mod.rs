use chrono::{Date, TimeZone, Utc};
use reqwest::{Body, Client, StatusCode};
use reqwest::header::{Accept, Connection, ConnectionOption, ContentType, UserAgent};
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
                        id: id,
                        kind: kind
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

        let request_data = request.encode().map_err(err_str)?;
        println!("Request: [\n{}\n]", str::from_utf8(&request_data).unwrap_or("[Invalid UTF-8]"));

        let client = Client::new();

        let mut response = client
            .post(request.url)
            .header(Accept(vec!["application/ofx".parse().unwrap()]))
            .header(Connection(vec![ConnectionOption::Close]))
            .header(ContentType("application/x-ofx".parse().unwrap()))
            .header(UserAgent::new("puccinia".to_string()))
            .body(Body::from(request_data))
            .send().map_err(err_str)?;

        let mut response_data = Vec::new();
        response.read_to_end(&mut response_data).map_err(err_str)?;
        println!("Response: [\n{}\n]", str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]"));

        match response.status() {
            StatusCode::Ok => {
                Ok(Response::decode(&response_data).map_err(err_str)?)
            },
            _ => {
                Err(format!("error code {}\n{}", response.status(), str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]")))
            }
        }
    }
}
