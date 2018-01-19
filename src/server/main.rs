#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate decimal;
extern crate diesel;
extern crate puccinia;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use puccinia::database::ConnectionMutex;
use puccinia::import::import;
use rocket_contrib::Template;
use std::env;
use std::fs::File;
use std::io::Read;

mod account;
mod index;
mod wallet;

fn main() {
    let mut config_tomls = Vec::new();

    for arg in env::args().skip(1) {
        let mut data = String::new();
        File::open(arg).unwrap().read_to_string(&mut data).unwrap();
        config_tomls.push(data);
    }

    if ! config_tomls.is_empty() {
        import(config_tomls.iter());
    }

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
