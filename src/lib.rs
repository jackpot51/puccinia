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

use bank::BankConfig;
use crypto::{Crypto, CryptoConfig};
use custom::{Custom, CustomConfig};
use ofx::Ofx;

pub mod bank;
pub mod crypto;
pub mod custom;
pub mod ofx;

pub struct Puccinia {
    pub bank: Vec<Box<Ofx>>,
    pub crypto: Vec<Box<Crypto>>,
    pub custom: Vec<Custom>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub bank: Vec<BankConfig>,
    #[serde(default)]
    pub crypto: Vec<CryptoConfig>,
    #[serde(default)]
    pub custom: Vec<CustomConfig>,
}

impl Config {
    pub fn build(self) -> Result<Puccinia, String> {
        let mut puccinia = Puccinia {
            bank: Vec::new(),
            crypto: Vec::new(),
            custom: Vec::new()
        };

        for bank in self.bank {
            puccinia.bank.push(bank.build()?);
        }

        for crypto in self.crypto {
            puccinia.crypto.push(crypto.build()?);
        }

        for custom in self.custom {
            puccinia.custom.push(custom.build()?);
        }

        Ok(puccinia)
    }
}
