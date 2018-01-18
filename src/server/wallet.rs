use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account};
use puccinia::database::schema::{wallets, accounts};
use rocket::State;
use rocket_contrib::Template;

#[get("/wallet/<id>")]
pub fn wallet(connection_mutex: State<ConnectionMutex>, id: String) -> Result<Template, String> {
    let connection = connection_mutex.lock();

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        accounts: Vec<Account>,
    }

    let wallet = wallets::table
        .find(&id)
        .first::<Wallet>(&*connection)
        .unwrap();


    let mut context = Context {
        wallet: wallet,
        accounts: Vec::new(),
    };

    let accounts = accounts::table
        .filter(accounts::wallet_id.eq(&id))
        .order(accounts::id.asc())
        .load::<Account>(&*connection)
        .unwrap();

    for account in accounts {
        context.accounts.push(account);
    }

    Ok(Template::render("wallet", &context))
}
