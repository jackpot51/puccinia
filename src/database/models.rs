use super::schema::{wallets, accounts, positions, position_transactions, transactions};

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "wallets"]
pub struct Wallet {
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "accounts"]
pub struct Account {
    pub wallet_id: String,
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "positions"]
pub struct Position {
    pub wallet_id: String,
    pub account_id: String,
    pub id: String,
    pub name: String,
    pub units: String,
    pub price: String,
    pub value: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "position_transactions"]
pub struct PositionTransaction {
    pub wallet_id: String,
    pub account_id: String,
    pub position_id: String,
    pub id: String,
    pub name: String,
    pub time: String,
    pub units: String,
    pub price: String,
    pub value: String,
}

#[derive(Insertable, Queryable, Serialize)]
#[table_name = "transactions"]
pub struct Transaction {
    pub wallet_id: String,
    pub account_id: String,
    pub id: String,
    pub name: String,
    pub time: String,
    pub amount: String,
}
