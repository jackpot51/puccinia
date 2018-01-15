extern crate decimal;
extern crate puccinia;
extern crate toml;

use decimal::d128;
use puccinia::Config;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut config_tomls = Vec::new();

    for arg in env::args().skip(1) {
        let mut data = String::new();
        File::open(arg).unwrap().read_to_string(&mut data).unwrap();
        config_tomls.push(data);
    }

    if config_tomls.is_empty() {
        config_tomls.push(include_str!("../example.toml").to_string());
    }

    let mut balances = Vec::new();

    for config_toml in config_tomls {
        let config: Config = toml::from_str(&config_toml).unwrap();
        let puccinia = config.build().unwrap();

        for (_id, bank) in &puccinia.bank {
            for account in bank.accounts().unwrap() {
                let amount = bank.amount(&account).unwrap();
                balances.push((format!("{}_{}_{}", bank.name(), account.kind, account.id), amount));
            }
        }

        for (_id, crypto) in &puccinia.crypto {
            let name = crypto.name();
            let address = crypto.address();
            let amount = crypto.amount().unwrap();
            let rate = crypto.rate().unwrap();
            println!("{}: {} @ {}", address, amount, rate);
            balances.push((format!("{}_{}", name, address), amount * rate));
        }

        for (_id, custom) in &puccinia.custom {
            let name = custom.name();
            let amount = custom.amount();
            balances.push((format!("{}", name), amount));
        }
    }

    let mut balances_unique = BTreeMap::new();
    for (account_id, amount) in balances {
        if let Some(previous_amount) = balances_unique.insert(account_id.clone(), amount) {
            println!("{}: replacing {} with {}", account_id, previous_amount, amount);
        }
    }

    println!("");

    println!("ACCOUNT: AMOUNT");

    let mut total = d128::zero();
    for (account_id, amount) in &balances_unique {
        println!("{}: {}", account_id, amount);
        total += *amount;
    }

    println!("TOTAL: {}", total);
}
