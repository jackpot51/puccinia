extern crate diesel;
extern crate puccinia;

use diesel::prelude::*;

use puccinia::database::{establish_connection};
use puccinia::database::models::{Position, NewPosition, Transaction, NewTransaction};
use puccinia::database::schema::{positions, transactions};

fn main() {
    let connection = establish_connection();

    diesel::delete(transactions::table)
        .execute(&connection)
        .unwrap();

    for i in 0..10 {
        diesel::insert_into(positions::table)
            .values(&NewPosition {
                name: &format!("NAME {}", i),
                units: &format!("UNITS {}", i),
                price: &format!("PRICE {}", i),
            })
            .execute(&connection)
            .unwrap();
    }

    for i in 0..10 {
        diesel::insert_into(transactions::table)
            .values(&NewTransaction {
                name: &format!("NAME {}", i),
                amount: &format!("AMOUNT {}", i),
            })
            .execute(&connection)
            .unwrap();
    }

    let positions = positions::table
       .load::<Position>(&connection)
       .unwrap();

    let transactions = transactions::table
       .load::<Transaction>(&connection)
       .unwrap();

    println!("Displaying {} positions", positions.len());
    for position in positions {
        println!("{}: {} x {}", position.name, position.units, position.price);
    }

    println!("Displaying {} transactions", transactions.len());
    for transaction in transactions {
        println!("{}: {}", transaction.name, transaction.amount);
    }
}
