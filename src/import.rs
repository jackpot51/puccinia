use chrono::{Duration, Local, TimeZone, Utc};
use diesel;
use diesel::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use toml;

use Config;
use database::{establish_connection};
use database::models::{Wallet, Account, Position, PositionPrice, PositionTransaction, Transaction, Transfer};
use database::schema::{wallets, accounts, positions, position_prices, position_transactions, transactions, transfers};

#[derive(Deserialize, Serialize)]
pub struct TransferConfig {
    pub name: String,
    pub from: String,
    pub from_wallet: Option<String>,
    pub from_account: Option<String>,
    pub to: String,
    pub to_wallet: Option<String>,
    pub to_account: Option<String>,
    #[serde(default)]
    pub instant: bool,
}

pub fn import<S: AsRef<str>, I: Iterator<Item=S>>(config_tomls: I) {
    let connection = establish_connection();

    // In the future, a `stale` boolean could identify which rows to delete in a safer manner

    // Deleted to ensure that only current wallets are listed
    diesel::delete(wallets::table)
        .execute(&connection)
        .unwrap();

    // Deleted to ensure that only current accounts are listed
    diesel::delete(accounts::table)
        .execute(&connection)
        .unwrap();

    // Deleted to ensure that only current positions are listed
    diesel::delete(positions::table)
        .execute(&connection)
        .unwrap();

    // Deleted to ensure that only current transfers are listed
    diesel::delete(transfers::table)
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
                        name: match &account.name {
                            Some(some) => some.clone(),
                            None => format!("{}_{}", account.id, account.kind),
                        },
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
                            id: position.id.clone(),
                            name: position.name,
                            units: format!("{}", position.units),
                            price: format!("{}", position.price),
                            value: format!("{}", position.value),
                        })
                        .execute(&connection)
                        .unwrap();

                    for transaction in position.transactions {
                        diesel::replace_into(position_transactions::table)
                            .values(&PositionTransaction {
                                wallet_id: id.to_string(),
                                account_id: account.id.clone(),
                                position_id: position.id.clone(),
                                id: transaction.id,
                                name: transaction.name,
                                time: format!("{}", transaction.time.format("%Y-%m-%d")),
                                units: format!("{}", transaction.units),
                                price: format!("{}", transaction.price),
                                value: format!("{}", transaction.value),
                            })
                            .execute(&connection)
                            .unwrap();
                    }

                    if position.id != "balance" {
                        println!("{}: checking cached prices", position.id);

                        let yesterday = Local::today() - Duration::days(1);
                        let time = format!("{}", yesterday.format("%Y-%m-%d"));
                        let download = match position_prices::table
                            .find((&id, &account.id, &position.id, &time))
                            .first::<PositionPrice>(&connection) {
                            Ok(price) => {
                                println!("{}: found cached price on {}", position.id, price.time);
                                false
                            },
                            Err(err) => {
                                println!("{}: failed to find cached price on {}: {}", position.id, time, err);
                                true
                            }
                        };

                        if download {
                            println!("{}: downloading prices", position.id);
                            match puccinia.alpha_vantage.daily(&position.id, false) {
                                Ok(data) => {
                                    println!("{}: downloaded {} prices", position.id, data.series.len());
                                    for (time, point) in data.series {
                                        diesel::replace_into(position_prices::table)
                                            .values(&PositionPrice {
                                                wallet_id: id.to_string(),
                                                account_id: account.id.clone(),
                                                position_id: position.id.clone(),
                                                time: time,
                                                price: point.close,
                                            })
                                            .execute(&connection)
                                            .unwrap();
                                    }
                                },
                                Err(err) => {
                                    println!("{}: failed to download prices: {}", position.id, err);
                                }
                            }
                        }
                    }
                }

                for transaction in statement.transactions {
                    diesel::replace_into(transactions::table)
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

            println!("{}: checking cached prices", kind);

            let yesterday = Local::today() - Duration::days(1);
            let time = format!("{}", yesterday.format("%Y-%m-%d"));
            let download = match position_prices::table
                .find((&id, &address, &kind, &time))
                .first::<PositionPrice>(&connection) {
                Ok(price) => {
                    println!("{}: found cached price on {}", kind, price.time);
                    false
                },
                Err(err) => {
                    println!("{}: failed to find cached price on {}: {}", kind, time, err);
                    true
                }
            };

            if download {
                let av_kind = match kind {
                    "bitcoin" => "BTC",
                    other => other
                };

                println!("{}: downloading prices", av_kind);
                match puccinia.alpha_vantage.crypto_daily(&av_kind) {
                    Ok(data) => {
                        println!("{}: downloaded {} prices", av_kind, data.series.len());
                        for (time, point) in data.series {
                            diesel::replace_into(position_prices::table)
                                .values(&PositionPrice {
                                    wallet_id: id.to_string(),
                                    account_id: address.to_string(),
                                    position_id: kind.to_string(),
                                    time: time,
                                    price: point.close,
                                })
                                .execute(&connection)
                                .unwrap();
                        }
                    },
                    Err(err) => {
                        println!("{}: failed to download prices: {}", av_kind, err);
                    }
                }
            }
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

        let transactions = transactions::table
            .order((transactions::time.asc(), transactions::id.asc()))
            .load::<Transaction>(&connection)
            .unwrap();
        for transfer in &puccinia.transfer {
            println!("{} from '{}' to '{}'", transfer.name, transfer.from, transfer.to);

            let mut froms = Vec::new();
            for from in transactions.iter() {
                if from.name != transfer.from {
                    continue;
                }

                if let Some(wallet_id) = &transfer.from_wallet {
                    if &from.wallet_id != wallet_id {
                        continue;
                    }
                }

                if let Some(account_id) = &transfer.from_account {
                    if &from.account_id != account_id {
                        continue;
                    }
                }

                let from_amount = match Decimal::from_str(&from.amount) {
                    Ok(ok) => ok,
                    Err(err) => {
                        println!("  Invalid amount for 'from' transaction: {}: {}", from.amount, err);
                        continue;
                    }
                };

                if from_amount.is_sign_positive() {
                    println!("  Positive amount for 'from' transaction: {}", from.amount);
                    continue;
                }

                froms.push((from, from_amount));
            }

            let mut tos = Vec::new();
            for to in transactions.iter() {
                if to.name != transfer.to {
                    continue;
                }

                if let Some(wallet_id) = &transfer.to_wallet {
                    if &to.wallet_id != wallet_id {
                        continue;
                    }
                }

                if let Some(account_id) = &transfer.to_account {
                    if &to.account_id != account_id {
                        continue;
                    }
                }

                let to_amount = match Decimal::from_str(&to.amount) {
                    Ok(ok) => ok,
                    Err(err) => {
                        println!("  Invalid amount for 'to' transaction: {}: {}", to.amount, err);
                        continue;
                    }
                };

                if to_amount.is_sign_negative() {
                    println!("  Negative amount for 'to' transaction: {}", to.amount);
                    continue;
                }

                tos.push((to, to_amount));
            }

            let mut from_i = 0;
            while from_i < froms.len() {
                let (from, from_amount) = froms[from_i];

                let mut matched = false;
                let mut to_i = 0;
                while !matched && to_i < tos.len() {
                    let (to, to_amount) = tos[to_i];

                    if from_amount == -to_amount && (!transfer.instant || from.time == to.time) {
                        matched = true;

                        println!("  From: {}: {} > {}: {}", from.time, from.wallet_id, from.account_id, from.amount);
                        println!("  To:   {}: {} > {}: {}", to.time, to.wallet_id, to.account_id, to.amount);

                        diesel::insert_into(transfers::table)
                            .values(&Transfer {
                                from_wallet_id: from.wallet_id.to_string(),
                                from_account_id: from.account_id.to_string(),
                                from_id: from.id.to_string(),
                                to_wallet_id: to.wallet_id.to_string(),
                                to_account_id: to.account_id.to_string(),
                                to_id: to.id.to_string(),
                            })
                            .execute(&connection)
                            .unwrap();
                    }

                    if matched {
                        tos.remove(to_i);
                    } else {
                        to_i += 1;
                    }
                }

                if matched {
                    froms.remove(from_i);
                } else {
                    from_i += 1;
                }
            }

            for (from, _) in froms {
                println!("  From not matched: {}: {} > {}: {}", from.time, from.wallet_id, from.account_id, from.amount);
            }

            for (to, _) in tos {
                println!("  To not matched: {}: {} > {}: {}", to.time, to.wallet_id, to.account_id, to.amount);
            }
        }
    }
}
