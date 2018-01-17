extern crate chrono;
extern crate diesel;
extern crate puccinia;
extern crate toml;

use chrono::{Utc, TimeZone};
use diesel::prelude::*;
use puccinia::Config;
use puccinia::database::{establish_connection};
use puccinia::database::models::{Position, NewPosition, Transaction, NewTransaction};
use puccinia::database::schema::{positions, transactions};
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

    diesel::delete(positions::table)
        .execute(&connection)
        .unwrap();

    diesel::delete(transactions::table)
        .execute(&connection)
        .unwrap();

    for config_toml in config_tomls {
        let config: Config = toml::from_str(&config_toml).unwrap();
        let puccinia = config.build().unwrap();

        for (_id, bank) in &puccinia.bank {
            for account in bank.accounts().unwrap() {
                let amount = bank.amount(&account).unwrap();

                diesel::insert_into(positions::table)
                    .values(&NewPosition {
                        name: bank.name(),
                        units: &format!("{}", amount),
                        price: &format!("1"),
                    })
                    .execute(&connection)
                    .unwrap();

                for transaction in bank.transactions(&account, Utc.ymd(2017, 12, 1), Utc::today()).unwrap() {
                    diesel::insert_into(transactions::table)
                        .values(&NewTransaction {
                            name: &transaction.name,
                            time: &format!("{}", transaction.time.format("%Y-%m-%d")),
                            amount: &format!("{}", transaction.amount),
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
