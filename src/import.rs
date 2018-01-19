use chrono::{Utc, TimeZone};
use diesel;
use diesel::prelude::*;
use toml;

use Config;
use database::{establish_connection};
use database::models::{Wallet, Account, Position, Transaction};
use database::schema::{wallets, accounts, positions, transactions};

pub fn import<S: AsRef<str>, I: Iterator<Item=S>>(config_tomls: I) {
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
        let config: Config = toml::from_str(config_toml.as_ref()).unwrap();
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

                let statement = bank.statement(&account, Utc.ymd(2017, 12, 1), Utc::today()).unwrap();

                for position in statement.positions {
                    diesel::insert_into(positions::table)
                        .values(&Position {
                            wallet_id: id.to_string(),
                            account_id: account.id.clone(),
                            id: position.id,
                            name: position.name,
                            units: format!("{}", position.units),
                            price: format!("{}", position.price),
                            value: format!("{}", position.value),
                        })
                        .execute(&connection)
                        .unwrap();
                }

                for transaction in statement.transactions {
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
}
