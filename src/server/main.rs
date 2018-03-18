#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate puccinia;
extern crate rocket;
extern crate rocket_contrib;
extern crate rust_decimal;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use puccinia::database::ConnectionMutex;
use puccinia::import::import;
use rocket_contrib::Template;
use rocket::response::NamedFile;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod account;
mod index;
mod json;
mod transaction;
mod wallet;

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(file)).ok()
}

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
            transaction::transaction_all,
            transaction::transaction,
            json::json,
            static_files,
        ])
        .attach(Template::fairing())
        .manage(ConnectionMutex::new())
        .launch();
}
