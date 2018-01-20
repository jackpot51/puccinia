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

                let start = Utc.ymd(2017, 1, 1);
                let end = Utc::today();
                let statement = bank.statement(&account, Some(start), Some(end)).unwrap();

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

        for (id, crypto) in &puccinia.crypto {
            diesel::insert_into(wallets::table)
                .values(&Wallet {
                    id: id.to_string(),
                    name: crypto.name().to_string()
                })
                .execute(&connection)
                .unwrap();

            let address = crypto.address();
            diesel::insert_into(accounts::table)
                .values(&Account {
                    wallet_id: id.to_string(),
                    id: address.to_string(),
                    name: format!("{}", address)
                })
                .execute(&connection)
                .unwrap();

            let kind = crypto.kind();
            let amount = crypto.amount().unwrap();
            let rate = crypto.rate().unwrap();
            diesel::insert_into(positions::table)
                .values(&Position {
                    wallet_id: id.to_string(),
                    account_id: address.to_string(),
                    id: kind.to_string(),
                    name: kind.to_string(),
                    units: format!("{}", amount),
                    price: format!("{}", rate),
                    value: format!("{}", amount * rate),
                })
                .execute(&connection)
                .unwrap();
        }

        for (id, custom) in &puccinia.custom {
            let name = custom.name();
            diesel::insert_into(wallets::table)
                .values(&Wallet {
                    id: id.to_string(),
                    name: name.to_string()
                })
                .execute(&connection)
                .unwrap();

            diesel::insert_into(accounts::table)
                .values(&Account {
                    wallet_id: id.to_string(),
                    id: id.to_string(),
                    name: name.to_string()
                })
                .execute(&connection)
                .unwrap();

            let amount = custom.amount();
            diesel::insert_into(positions::table)
                .values(&Position {
                    wallet_id: id.to_string(),
                    account_id: id.to_string(),
                    id: id.to_string(),
                    name: name.to_string(),
                    units: format!("{}", amount),
                    price: format!("{}", 1),
                    value: format!("{}", amount),
                })
                .execute(&connection)
                .unwrap();
        }
    }
}
