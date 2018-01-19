use decimal::d128;
use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account, Position};
use puccinia::database::schema::{wallets, accounts, positions};
use rocket::State;
use rocket_contrib::Template;
use std::str::FromStr;

#[get("/wallet/<id>")]
pub fn wallet(connection_mutex: State<ConnectionMutex>, id: String) -> Result<Template, String> {
    let connection = connection_mutex.lock();

    #[derive(Serialize)]
    struct AccountContext {
        account: Account,
        total: d128,
    }

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        total: d128,
        accounts: Vec<AccountContext>,
    }

    let wallet = wallets::table
        .find(&id)
        .first::<Wallet>(&*connection)
        .unwrap();


    let mut context = Context {
        wallet: wallet,
        total: d128!(0),
        accounts: Vec::new(),
    };

    let accounts = accounts::table
        .filter(accounts::wallet_id.eq(&id))
        .order(accounts::id.asc())
        .load::<Account>(&*connection)
        .unwrap();

    for account in accounts {
        let mut total = d128!(0);

        let positions = positions::table
            .filter(positions::wallet_id.eq(&id))
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

        context.accounts.push(AccountContext {
            account: account,
            total: total
        });
    }

    Ok(Template::render("wallet", &context))
}
