use actix_web::{error, Error, Json, State};
use diesel::prelude::*;
use puccinia::database::models::{Wallet, Account, Position, Transaction};
use puccinia::database::schema::{wallets, accounts, positions, transactions};
use std::sync::Arc;
use super::AppState;

#[derive(Serialize)]
pub struct JsonDump {
    wallets: Vec<Wallet>,
    accounts: Vec<Account>,
    positions: Vec<Position>,
    transactions: Vec<Transaction>,
}

pub fn json(state: State<Arc<AppState>>) -> Result<Json<JsonDump>, Error> {
    let connection = state.db.lock()
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let wallets = wallets::table
        .order(wallets::id.asc())
        .load::<Wallet>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let accounts = accounts::table
        .order(accounts::id.asc())
        .load::<Account>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let positions = positions::table
        .order(positions::id.asc())
        .load::<Position>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let transactions = transactions::table
        .order((transactions::time.asc(), transactions::id.asc()))
        .load::<Transaction>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    Ok(Json(JsonDump {
        wallets,
        accounts,
        positions,
        transactions
    }))
}
