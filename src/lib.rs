extern crate chrono;
extern crate coinnect;
#[macro_use]
extern crate decimal;
extern crate hyper;
extern crate hyper_native_tls;
extern crate mime;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate xml;

use std::collections::BTreeMap;

use bank::{Bank, BankConfig};
use crypto::{Crypto, CryptoConfig};
use custom::{Custom, CustomConfig};

pub mod bank;
pub mod crypto;
pub mod custom;
pub mod ofx;

pub struct Puccinia {
    pub bank: BTreeMap<String, Box<Bank>>,
    pub crypto: BTreeMap<String, Box<Crypto>>,
    pub custom: BTreeMap<String, Custom>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub bank: BTreeMap<String, BankConfig>,
    #[serde(default)]
    pub crypto: BTreeMap<String, CryptoConfig>,
    #[serde(default)]
    pub custom: BTreeMap<String, CustomConfig>,
}

impl Config {
    pub fn build(self) -> Result<Puccinia, String> {
        let mut puccinia = Puccinia {
            bank: BTreeMap::new(),
            crypto: BTreeMap::new(),
            custom: BTreeMap::new()
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
