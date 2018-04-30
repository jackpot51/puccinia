use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::{Wallet, Account, Position};
use puccinia::database::schema::{wallets, accounts, positions};
use rocket::State;
use rocket_contrib::Template;
use rust_decimal::Decimal;
use std::str::FromStr;

#[get("/wallet/<id>")]
pub fn wallet(connection_mutex: State<ConnectionMutex>, id: String) -> Result<Template, String> {
    let connection = connection_mutex.lock().map_err(|err| format!("{}", err))?;

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
        .map_err(|err| format!("{}", err))?;


    let mut context = Context {
        wallet: wallet,
        total: Decimal::new(0, 0),
        accounts: Vec::new(),
    };

    let accounts = accounts::table
        .filter(accounts::wallet_id.eq(&id))
        .order(accounts::id.asc())
        .load::<Account>(&*connection)
        .map_err(|err| format!("{}", err))?;

    for account in accounts {
        let mut total = Decimal::new(0, 0);

        let positions = positions::table
            .filter(positions::wallet_id.eq(&id))
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

        context.accounts.push(AccountContext {
            account: account,
            total: total.round_dp(2)
        });
    }

    context.total = context.total.round_dp(2);

    Ok(Template::render("wallet", &context))
}
