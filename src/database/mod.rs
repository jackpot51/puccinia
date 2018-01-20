use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::sync::{LockResult, Mutex, MutexGuard};

pub mod schema;
pub mod models;

pub struct ConnectionMutex(Mutex<SqliteConnection>);

impl ConnectionMutex {
    pub fn new() -> Self {
        ConnectionMutex(Mutex::new(establish_connection()))
    }

    pub fn lock<'a>(&'a self) -> LockResult<MutexGuard<'a, SqliteConnection>> {
        self.0.lock()
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
