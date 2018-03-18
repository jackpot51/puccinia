use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account, Position, Transaction};
use puccinia::database::schema::{wallets, accounts, positions, transactions};
use rocket::State;
use rocket_contrib::Json;

#[derive(Serialize)]
pub struct JsonDump {
    wallets: Vec<Wallet>,
    accounts: Vec<Account>,
    positions: Vec<Position>,
    transactions: Vec<Transaction>,
}

#[get("/json")]
pub fn json(connection_mutex: State<ConnectionMutex>) -> Result<Json<JsonDump>, String> {
    let connection = connection_mutex.lock().map_err(|err| format!("{}", err))?;

    let wallets = wallets::table
        .order(wallets::id.asc())
        .load::<Wallet>(&*connection)
        .map_err(|err| format!("{}", err))?;

    let accounts = accounts::table
        .order(accounts::id.asc())
        .load::<Account>(&*connection)
        .map_err(|err| format!("{}", err))?;

    let positions = positions::table
        .order(positions::id.asc())
        .load::<Position>(&*connection)
        .map_err(|err| format!("{}", err))?;

    let transactions = transactions::table
        .order((transactions::time.asc(), transactions::id.asc()))
        .load::<Transaction>(&*connection)
        .map_err(|err| format!("{}", err))?;

    Ok(Json(JsonDump {
        wallets,
        accounts,
        positions,
        transactions
    }))
}
