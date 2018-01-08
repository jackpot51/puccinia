use chrono::{Date, Utc};
use decimal::d128;
use hyper::Client;
use hyper::client::Body;
use hyper::net::HttpsConnector;
use hyper::header::{Accept, Connection, ConnectionOption, ContentType, UserAgent};
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use std::{str, time};
use std::io::Read;
use std::str::FromStr;

use bank::BankAccount;

pub use self::request::Request;
pub use self::response::Response;

mod request;
mod response;

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

    fn ofx_amount(&self, account_id: &str, account_type: &str) -> Result<d128, String> {
        let response = self.ofx(account_id, account_type, None, None)?;

        let mut total = d128::zero();

        if let Some(balance) = response.balance {
            if let Some(amount) = balance.amount {
                total += d128::from_str(&amount).map_err(|_err| {
                    format!("invalid decimal: {}", amount)
                })?;
            }
        }

        for position in response.positions {
            if let Some(market_value) = position.market_value {
                total += d128::from_str(&market_value).map_err(|_err| {
                    format!("invalid decimal: {}", market_value)
                })?;
            }
        }

        Ok(total)
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

        let request_data = request.encode().map_err(|err| format!("{}", err))?;
        println!("Request: [\n{}\n]", str::from_utf8(&request_data).unwrap_or("[Invalid UTF-8]"));

        let tls_client = NativeTlsClient::new().map_err(|err| format!("{}", err))?;
        let connector = HttpsConnector::new(tls_client);
        let mut client = Client::with_connector(connector);
        client.set_read_timeout(Some(time::Duration::new(5, 0)));
        client.set_write_timeout(Some(time::Duration::new(5, 0)));

        let mut response = client
            .post(request.url)
            .header(Accept(vec!["application/ofx".parse().unwrap()]))
            .header(Connection(vec![ConnectionOption::Close]))
            .header(ContentType("application/x-ofx".parse().unwrap()))
            .header(UserAgent("puccinia".to_string()))
            .body(Body::BufBody(&request_data, request_data.len()))
            .send().map_err(|err| format!("{}", err))?;

        let mut response_data = Vec::new();
        response.read_to_end(&mut response_data).map_err(|err| format!("{}", err))?;
        println!("Response: [\n{}\n]", str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]"));

        match response.status {
            StatusCode::Ok => {
                Ok(Response::decode(&response_data).map_err(|err| format!("{}", err))?)
            },
            _ => {
                Err(format!("error code {}\n{}", response.status, str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]")))
            }
        }
    }
}
