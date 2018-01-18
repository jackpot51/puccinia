#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate decimal;
extern crate diesel;
extern crate puccinia;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use puccinia::database::ConnectionMutex;
use rocket_contrib::Template;

mod account;
mod index;
mod wallet;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index::index,
            wallet::wallet,
            account::account,
        ])
        .attach(Template::fairing())
        .manage(ConnectionMutex::new())
        .launch();
}
