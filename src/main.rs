extern crate hyper;
extern crate hyper_rustls;
extern crate mime;
extern crate puccinia;

use hyper::Client;
use hyper::client::Body;
use hyper::net::HttpsConnector;
use hyper::header::{Accept, Connection, ConnectionOption, ContentLength, ContentType, UserAgent};
use hyper::status::StatusCode;
use puccinia::ofx::Request;
use std::{io, process, str, time};
use std::io::{Read, Write};

fn main() {
    let request = Request {
        url: "url",
        ofx_ver: "102",

        time: "time",
        user: "user",
        password: "password",
        language: "ENG",
        fid: "fid",
        fid_org: "fid_org",
        app_id: "QWIN",
        app_ver: "1700",
        client_id: "client_id",

        bank_id: "bank_id",
        account_id: "account_id",
        account_type: "account_type",
        start: "start",
        end: "end"
    };

    let data = request.encode().unwrap();
    println!("{}", str::from_utf8(&data).unwrap());


    let mut output = io::stdout();
    let mut stderr = io::stderr();

    let mut client = Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
    client.set_read_timeout(Some(time::Duration::new(5, 0)));
    client.set_write_timeout(Some(time::Duration::new(5, 0)));

    match client
        .post(request.url)
        .header(Accept(vec!["application/ofx".parse().unwrap()]))
        .header(Connection(vec![ConnectionOption::Close]))
        .header(ContentType("application/x-ofx".parse().unwrap()))
        .header(UserAgent("puccinia".to_string()))
        .body(Body::BufBody(&data, data.len()))
        .send() {
        Ok(mut response) => match response.status {
            StatusCode::Ok => {
                let mut count = 0;
                let length = response.headers.get::<ContentLength>().map_or(0, |h| h.0 as usize);

                loop {
                    let mut buf = [0; 8192];
                    let res = match response.read(&mut buf) {
                        Ok(res) => res,
                        Err(err) => {
                            writeln!(stderr, "wget: failed to read data: {}", err).unwrap();
                            process::exit(1);
                        }
                    };
                    if res == 0 {
                        break;
                    }
                    count += match output.write(&buf[.. res]) {
                        Ok(res) => res,
                        Err(err) => {
                            writeln!(stderr, "wget: failed to write data: {}", err).unwrap();
                            process::exit(1);
                        }
                    };
                }
            },
            _ => {
                let _ = writeln!(stderr, "wget: failed to receive request: {}", response.status);
                process::exit(1);
            }
        },
        Err(err) => {
            let _ = writeln!(stderr, "wget: failed to send request: {}", err);
            process::exit(1);
        }
    }
}
