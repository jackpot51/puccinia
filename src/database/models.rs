use super::schema::{wallets, accounts, positions, transactions};

#[derive(Insertable, Queryable)]
#[table_name = "wallets"]
pub struct Wallet {
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "accounts"]
pub struct Account {
    pub wallet_id: String,
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "positions"]
pub struct Position {
    pub wallet_id: String,
    pub account_id: String,
    pub id: String,
    pub name: String,
    pub units: String,
    pub price: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "transactions"]
pub struct Transaction {
    pub wallet_id: String,
    pub account_id: String,
    pub id: String,
    pub name: String,
    pub time: String,
    pub amount: String,
}
