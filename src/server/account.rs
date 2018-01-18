use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account, Position, Transaction};
use puccinia::database::schema::{wallets, accounts, positions, transactions};
use rocket::State;
use rocket_contrib::Template;

#[get("/account/<wallet_id>/<id>")]
pub fn account(connection_mutex: State<ConnectionMutex>, wallet_id: String, id: String) -> Result<Template, String> {
    let connection = connection_mutex.lock();

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        account: Account,
        positions: Vec<Position>,
        transactions: Vec<Transaction>,
    }

    let wallet = wallets::table
        .find(&wallet_id)
        .first::<Wallet>(&*connection)
        .unwrap();

    let account = accounts::table
        .find((&wallet_id, &id))
        .first::<Account>(&*connection)
        .unwrap();

    let mut context = Context {
        wallet: wallet,
        account: account,
        positions: Vec::new(),
        transactions: Vec::new()
    };

    let positions = positions::table
        .filter(positions::wallet_id.eq(&wallet_id))
        .filter(positions::account_id.eq(&id))
        .order(positions::id.asc())
        .load::<Position>(&*connection)
        .unwrap();

    for position in positions {
        context.positions.push(position);
    }

    let transactions = transactions::table
        .filter(transactions::wallet_id.eq(&wallet_id))
        .filter(transactions::account_id.eq(&id))
        .order(transactions::time.asc())
        .load::<Transaction>(&*connection)
        .unwrap();

    for transaction in transactions {
        context.transactions.push(transaction);
    }

    Ok(Template::render("account", &context))
}
