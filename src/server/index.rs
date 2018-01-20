use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account, Position};
use puccinia::database::schema::{wallets, accounts, positions};
use rocket::State;
use rocket_contrib::Template;
use rust_decimal::Decimal;
use std::str::FromStr;

#[get("/")]
pub fn index(connection_mutex: State<ConnectionMutex>) -> Result<Template, String> {
    let connection = connection_mutex.lock().map_err(|err| format!("{}", err))?;

    #[derive(Serialize)]
    struct WalletContext {
        wallet: Wallet,
        total: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        wallets: Vec<WalletContext>,
        total: Decimal,
    }

    let mut context = Context {
        wallets: Vec::new(),
        total: Decimal::new(0, 0),
    };

    let wallets = wallets::table
        .order(wallets::id.asc())
        .load::<Wallet>(&*connection)
        .map_err(|err| format!("{}", err))?;

    for wallet in wallets {
        let mut total = Decimal::new(0, 0);

        let accounts = accounts::table
            .filter(accounts::wallet_id.eq(&wallet.id))
            .order(accounts::id.asc())
            .load::<Account>(&*connection)
            .map_err(|err| format!("{}", err))?;

        for account in accounts {
            let positions = positions::table
                .filter(positions::wallet_id.eq(&wallet.id))
                .filter(positions::account_id.eq(&account.id))
                .order(positions::id.asc())
                .load::<Position>(&*connection)
                .map_err(|err| format!("{}", err))?;

            for position in positions {
                if let Ok(value) = Decimal::from_str(&position.value) {
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

    Ok(Template::render("index", &context))
}
