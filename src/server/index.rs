use actix_web::{error, Responder, State};
use diesel::prelude::*;
use puccinia::database::models::{Wallet, Account, Position};
use puccinia::database::schema::{wallets, accounts, positions};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use super::AppState;

pub fn index(state: State<Arc<AppState>>) -> impl Responder {
    let connection = state.db.lock()
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

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
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    for wallet in wallets {
        let mut total = Decimal::new(0, 0);

        let accounts = accounts::table
            .filter(accounts::wallet_id.eq(&wallet.id))
            .order(accounts::id.asc())
            .load::<Account>(&*connection)
            .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

        for account in accounts {
            let positions = positions::table
                .filter(positions::wallet_id.eq(&wallet.id))
                .filter(positions::account_id.eq(&account.id))
                .order(positions::id.asc())
                .load::<Position>(&*connection)
                .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

            for position in positions {
                if let Ok(value) = Decimal::from_str(&position.value) {
                    context.total += value;
                    total += value;
                }
            }
        }

        context.wallets.push(WalletContext {
            wallet: wallet,
            total: total.round_dp(2),
        });
    }

    context.total = context.total.round_dp(2);

    state.templates.render("index", &context)
}
