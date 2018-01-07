use chrono::TimeZone;
use hyper::Client;
use hyper::client::Body;
use hyper::net::HttpsConnector;
use hyper::header::{Accept, Connection, ConnectionOption, ContentType, UserAgent};
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use std::{str, time};
use std::fmt::Display;
use std::io::Read;

pub use self::request::Request;
pub use self::response::Response;

pub mod bank;
mod request;
mod response;

pub fn ofx<'a, Tz: TimeZone>(request: Request<'a, Tz>) -> Result<Response, String> where Tz::Offset: Display {
    let request_data = request.encode().map_err(|err| format!("{}", err))?;

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

    match response.status {
        StatusCode::Ok => {
            Ok(Response::decode(&response_data).map_err(|err| format!("{}", err))?)
        },
        _ => {
            Err(format!("error code {}\n{}", response.status, str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]")))
        }
    }
}
