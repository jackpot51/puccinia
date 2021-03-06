extern crate alpha_vantage;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate mime;
extern crate rand;
extern crate reqwest;
extern crate rust_decimal;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate uuid;
extern crate xml;

use alpha_vantage::AlphaVantage;
use std::collections::BTreeMap;

use bank::{Bank, BankConfig};
use crypto::{Crypto, CryptoConfig};
use custom::{Custom, CustomConfig};
use import::TransferConfig;

pub mod bank;
pub mod crypto;
pub mod custom;
pub mod database;
pub mod import;
pub mod ofx;

// Helper function for errors
pub (crate) fn err_str<E: ::std::fmt::Display>(err: E) -> String {
    format!("{}", err)
}

pub struct Puccinia {
    pub alpha_vantage: AlphaVantage,
    pub bank: BTreeMap<String, Box<dyn Bank>>,
    pub crypto: BTreeMap<String, Box<dyn Crypto>>,
    pub custom: BTreeMap<String, Custom>,
    pub transfer: Vec<TransferConfig>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub alpha_vantage: String,
    #[serde(default)]
    pub bank: BTreeMap<String, BankConfig>,
    #[serde(default)]
    pub crypto: BTreeMap<String, CryptoConfig>,
    #[serde(default)]
    pub custom: BTreeMap<String, CustomConfig>,
    #[serde(default)]
    pub transfer: Vec<TransferConfig>,
}

impl Config {
    pub fn build(self) -> Result<Puccinia, String> {
        let mut puccinia = Puccinia {
            alpha_vantage: AlphaVantage::new(&self.alpha_vantage),
            bank: BTreeMap::new(),
            crypto: BTreeMap::new(),
            custom: BTreeMap::new(),
            transfer: self.transfer,
        };

        for (id, bank) in self.bank {
            puccinia.bank.insert(id, bank.build()?);
        }

        for (id, crypto) in self.crypto {
            puccinia.crypto.insert(id, crypto.build()?);
        }

        for (id, custom) in self.custom {
            puccinia.custom.insert(id, custom.build()?);
        }

        Ok(puccinia)
    }
}
