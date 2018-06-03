use actix_web::{error, Path, Responder, State};
use diesel::prelude::*;
use puccinia::database::models::{Wallet, Account, Position};
use puccinia::database::schema::{wallets, accounts, positions};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use super::AppState;

pub fn wallet(info: (Path<String>, State<Arc<AppState>>)) -> impl Responder {
    let connection = info.1.db.lock()
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;
    let id: String = info.0.into_inner();

    #[derive(Serialize)]
    struct AccountContext {
        account: Account,
        total: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        total: Decimal,
        accounts: Vec<AccountContext>,
    }

    let wallet = wallets::table
        .find(&id)
        .first::<Wallet>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;


    let mut context = Context {
        wallet: wallet,
        total: Decimal::new(0, 0),
        accounts: Vec::new(),
    };

    let accounts = accounts::table
        .filter(accounts::wallet_id.eq(&id))
        .order(accounts::id.asc())
        .load::<Account>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    for account in accounts {
        let mut total = Decimal::new(0, 0);

        let positions = positions::table
            .filter(positions::wallet_id.eq(&id))
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

        context.accounts.push(AccountContext {
            account: account,
            total: total.round_dp(2)
        });
    }

    context.total = context.total.round_dp(2);
    info.1.templates.render("wallet", &context)
}
