extern crate actix_web;
extern crate diesel;
extern crate handlebars;
extern crate puccinia;
extern crate rust_decimal;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use actix_web::{server::HttpServer, App, http::Method, fs::StaticFiles};
use puccinia::database::ConnectionMutex;
use puccinia::import::import;
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

mod account;
mod index;
mod json;
mod template;
mod transaction;
mod wallet;

use template::Templates;

pub struct AppState {
    pub db: ConnectionMutex,
    pub templates: Templates
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

    let state = Arc::new(AppState { db: ConnectionMutex::new(), templates: Templates::new() });

    let main_app = move || {
        App::with_state(state.clone())
            .route("/", Method::GET, index::index)
            .route("/wallet/{id}", Method::GET, wallet::wallet)
            .route("/account/{wallet_id}/{id}", Method::GET, account::account)
            .route("/transaction/{key}/{value}", Method::GET, transaction::transaction)
            .route("/transaction", Method::GET, transaction::transaction_all)
            .route("/json", Method::GET, json::json)
            .handler("/static", StaticFiles::new("static").show_files_listing())
    };

    // load ssl keys
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(main_app)
        .bind("127.0.0.1:8080").unwrap()
        // .server_ssl(builder).unwrap()
        .run();
}
