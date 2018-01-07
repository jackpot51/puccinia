extern crate decimal;
extern crate puccinia;
extern crate toml;

use decimal::d128;
use puccinia::Config;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

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

        for bank in &puccinia.bank {
            let response = bank.ofx("", "", None, None).unwrap();
            println!("{:#?}", response);

            for account in response.accounts {
                let account_id = account.id.unwrap();
                let account_type = account.kind.unwrap();
                let response = bank.ofx(&account_id, &account_type, None, None).unwrap();
                println!("{:#?}", response);

                if let Some(balance) = response.balance {
                    if let Some(amount) = balance.amount {
                        balances.push((format!("{}_{}_{}", bank.fid_org(), account_type, account_id), d128::from_str(&amount).unwrap()));
                    }
                }
            }
        }

        for crypto in &puccinia.crypto {
            let name = crypto.name();
            let address = crypto.address();
            let balance = crypto.balance().unwrap();
            let rate = crypto.rate().unwrap();
            println!("{}: {} @ {}", address, balance, rate);
            balances.push((format!("{}_{}", name, address), balance * rate));
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
