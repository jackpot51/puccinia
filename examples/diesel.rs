extern crate chrono;
extern crate diesel;
extern crate puccinia;
extern crate toml;

use chrono::{Utc, TimeZone};
use diesel::prelude::*;
use puccinia::Config;
use puccinia::database::{establish_connection};
use puccinia::database::models::{Wallet, Account, Position, Transaction};
use puccinia::database::schema::{wallets, accounts, positions, transactions};
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

    let connection = establish_connection();

    diesel::delete(wallets::table)
        .execute(&connection)
        .unwrap();

    diesel::delete(accounts::table)
        .execute(&connection)
        .unwrap();

    diesel::delete(positions::table)
        .execute(&connection)
        .unwrap();

    diesel::delete(transactions::table)
        .execute(&connection)
        .unwrap();

    for config_toml in config_tomls {
        let config: Config = toml::from_str(&config_toml).unwrap();
        let puccinia = config.build().unwrap();

        for (id, bank) in &puccinia.bank {
            diesel::insert_into(wallets::table)
                .values(&Wallet {
                    id: id.to_string(),
                    name: bank.name().to_string()
                })
                .execute(&connection)
                .unwrap();

            for account in bank.accounts().unwrap() {
                diesel::insert_into(accounts::table)
                    .values(&Account {
                        wallet_id: id.to_string(),
                        id: account.id.clone(),
                        name: format!("{}_{}", account.kind, account.id)
                    })
                    .execute(&connection)
                    .unwrap();

                let amount = bank.amount(&account).unwrap();

                diesel::insert_into(positions::table)
                    .values(&Position {
                        wallet_id: id.to_string(),
                        account_id: account.id.clone(),
                        id: "balance".to_string(),
                        name: "Balance".to_string(),
                        units: format!("{}", amount),
                        price: format!("1"),
                    })
                    .execute(&connection)
                    .unwrap();

                for transaction in bank.transactions(&account, Utc.ymd(2017, 12, 1), Utc::today()).unwrap() {
                    diesel::insert_into(transactions::table)
                        .values(&Transaction {
                            wallet_id: id.to_string(),
                            account_id: account.id.clone(),
                            id: transaction.id,
                            name: transaction.name,
                            time: format!("{}", transaction.time.format("%Y-%m-%d")),
                            amount: format!("{}", transaction.amount),
                        })
                        .execute(&connection)
                        .unwrap();
                }
            }
        }

        // for (_id, crypto) in &puccinia.crypto {
        //     let address = crypto.address();
        //     let amount = crypto.amount().unwrap();
        //     let rate = crypto.rate().unwrap();
        //     println!("{}: {} @ {}", address, amount, rate);
        //     balances.push((format!("{}_{}", crypto.kind(), address), amount * rate));
        // }
        //
        // for (id, custom) in &puccinia.custom {
        //     let amount = custom.amount();
        //     balances.push((format!("custom_{}", id), amount));
        // }
    }

    let positions = positions::table
       .load::<Position>(&connection)
       .unwrap();

    println!("Displaying {} positions", positions.len());
    for position in positions {
        println!("{}: {} x {}", position.name, position.units, position.price);
    }

    let transactions = transactions::table
        .order(transactions::time.asc())
        .load::<Transaction>(&connection)
        .unwrap();

    println!("Displaying {} transactions", transactions.len());
    for transaction in transactions {
        println!("{}: {}: {}", transaction.time, transaction.name, transaction.amount);
    }
}
