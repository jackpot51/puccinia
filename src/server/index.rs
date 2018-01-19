use decimal::d128;
use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account, Position};
use puccinia::database::schema::{wallets, accounts, positions};
use rocket::State;
use rocket_contrib::Template;
use std::str::FromStr;

#[get("/")]
pub fn index(connection_mutex: State<ConnectionMutex>) -> Template {
    let connection = connection_mutex.lock();

    #[derive(Serialize)]
    struct WalletContext {
        wallet: Wallet,
        total: d128,
    }

    #[derive(Serialize)]
    struct Context {
        wallets: Vec<WalletContext>,
        total: d128,
    }

    let mut context = Context {
        wallets: Vec::new(),
        total: d128!(0),
    };

    let wallets = wallets::table
        .order(wallets::id.asc())
        .load::<Wallet>(&*connection)
        .unwrap();

    for wallet in wallets {
        let mut total = d128!(0);

        let accounts = accounts::table
            .filter(accounts::wallet_id.eq(&wallet.id))
            .order(accounts::id.asc())
            .load::<Account>(&*connection)
            .unwrap();

        for account in accounts {
            let positions = positions::table
                .filter(positions::wallet_id.eq(&wallet.id))
                .filter(positions::account_id.eq(&account.id))
                .order(positions::id.asc())
                .load::<Position>(&*connection)
                .unwrap();

            for position in positions {
                if let Ok(value) = d128::from_str(&position.value) {
                    context.total += value;
                    total += value;
                }
            }
        }

        context.wallets.push(WalletContext {
            wallet: wallet,
            total: total,
        });
    }

    Template::render("index", &context)
}
