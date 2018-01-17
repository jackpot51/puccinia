use super::schema::{positions, transactions};

#[derive(Queryable)]
pub struct Position {
    pub id: i32,
    pub name: String,
    pub units: String,
    pub price: String,
}

#[derive(Insertable)]
#[table_name = "positions"]
pub struct NewPosition<'a> {
    pub name: &'a str,
    pub units: &'a str,
    pub price: &'a str,
}

#[derive(Queryable)]
pub struct Transaction {
    pub id: i32,
    pub name: String,
    pub time: String,
    pub amount: String,
}

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction<'a> {
    pub name: &'a str,
    pub time: &'a str,
    pub amount: &'a str,
}
