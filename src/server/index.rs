use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::Wallet;
use puccinia::database::schema::wallets;
use rocket::State;
use rocket_contrib::Template;

#[get("/")]
pub fn index(connection_mutex: State<ConnectionMutex>) -> Template {
    let connection = connection_mutex.lock();

    #[derive(Serialize)]
    struct Context {
        wallets: Vec<Wallet>,
    }

    let mut context = Context {
        wallets: Vec::new()
    };

    let wallets = wallets::table
        .order(wallets::id.asc())
        .load::<Wallet>(&*connection)
        .unwrap();

    for wallet in wallets {
        context.wallets.push(wallet);
    }

    Template::render("index", &context)
}
