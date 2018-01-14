#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate decimal;
extern crate puccinia;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use puccinia::Config;
use rocket_contrib::Template;
use std::{env, process};
use std::fs::File;
use std::io::Read;

mod bank;
mod crypto;
mod custom;
mod index;

fn main() {
    let config_toml = if let Some(arg) = env::args().nth(1) {
        let mut data = String::new();
        File::open(arg).unwrap().read_to_string(&mut data).unwrap();
        data
    } else {
        eprintln!("no configuration provided");
        process::exit(1);
    };

    let config: Config = toml::from_str(&config_toml).unwrap();
    let puccinia = config.build().unwrap();

    rocket::ignite()
        .mount("/", routes![
            index::index,
            bank::index,
            bank::account,
            crypto::crypto,
            custom::custom
        ])
        .attach(Template::fairing())
        .manage(puccinia)
        .launch();
}
