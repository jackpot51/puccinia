use actix_web::{error, Error, web::Json, web::Data};
use diesel::prelude::*;
use puccinia::database::models::{Wallet, Account, Position, PositionPrice, PositionTransaction, Transaction};
use puccinia::database::schema::{wallets, accounts, positions, position_prices, position_transactions, transactions};
use std::sync::Arc;
use super::AppState;

#[derive(Serialize)]
pub struct JsonDump {
    wallets: Vec<Wallet>,
    accounts: Vec<Account>,
    positions: Vec<Position>,
    position_prices: Vec<PositionPrice>,
    position_transactions: Vec<PositionTransaction>,
    transactions: Vec<Transaction>,
}

pub fn json(state: Data<Arc<AppState>>) -> Result<Json<JsonDump>, Error> {
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

    let position_prices = position_prices::table
        .order((position_prices::time.asc(), position_prices::position_id.asc()))
        .load::<PositionPrice>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let position_transactions = position_transactions::table
        .order((position_transactions::time.asc(), position_transactions::id.asc()))
        .load::<PositionTransaction>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let transactions = transactions::table
        .order((transactions::time.asc(), transactions::id.asc()))
        .load::<Transaction>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    Ok(Json(JsonDump {
        wallets,
        accounts,
        positions,
        position_prices,
        position_transactions,
        transactions
    }))
}
