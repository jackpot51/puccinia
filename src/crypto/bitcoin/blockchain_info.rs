use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::{Connection, ConnectionOption};
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use std::{str, time};
use std::io::Read;

pub struct BlockchainInfoApi;

impl BlockchainInfoApi {
    pub fn api(&self, path: &str) -> Result<String, String> {
        let url = format!("https://blockchain.info/q/{}", path);

        let tls_client = NativeTlsClient::new().map_err(|err| format!("{}", err))?;
        let connector = HttpsConnector::new(tls_client);
        let mut client = Client::with_connector(connector);
        client.set_read_timeout(Some(time::Duration::new(5, 0)));
        client.set_write_timeout(Some(time::Duration::new(5, 0)));

        let mut response = client
            .get(&url)
            .header(Connection(vec![ConnectionOption::Close]))
            .send().map_err(|err| format!("{}", err))?;

        let mut response_data = Vec::new();
        response.read_to_end(&mut response_data).map_err(|err| format!("{}", err))?;

        match response.status {
            StatusCode::Ok => {
                Ok(String::from_utf8(response_data).map_err(|err| format!("{}", err))?)
            },
            _ => {
                Err(format!("error code {}\n{}", response.status, str::from_utf8(&response_data).unwrap_or("[Invalid UTF-8]")))
            }
        }
    }

    pub fn address_balance(&self, address: &str) -> Result<String, String> {
        self.api(&format!("addressbalance/{}", address))
    }
}
