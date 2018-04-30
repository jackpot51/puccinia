use diesel::prelude::*;
use puccinia::database::ConnectionMutex;
use puccinia::database::models::Transaction;
use puccinia::database::schema::transactions;
use rocket::State;
use rocket_contrib::Template;
use rust_decimal::Decimal;
use std::str::FromStr;

#[get("/transaction")]
pub fn transaction_all(connection_mutex: State<ConnectionMutex>) -> Result<Template, String> {
    transaction(connection_mutex, String::new(), String::new())
}


#[get("/transaction/<key>/<value>")]
pub fn transaction(connection_mutex: State<ConnectionMutex>, key: String, value: String) -> Result<Template, String> {
    let connection = connection_mutex.lock().map_err(|err| format!("{}", err))?;

    #[derive(Serialize)]
    struct TransactionContext {
        transaction: Transaction,
        total: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        total: Decimal,
        input: Decimal,
        output: Decimal,
        transactions: Vec<TransactionContext>,
    }

    let transactions = match key.as_str() {
        "" => {
            transactions::table
                .order((transactions::time.desc(), transactions::id.desc()))
                .load::<Transaction>(&*connection)
                .map_err(|err| format!("{}", err))?
        },
        "time" => {
            transactions::table
                .filter(transactions::time.eq(&value))
                .order((transactions::time.desc(), transactions::id.desc()))
                .load::<Transaction>(&*connection)
                .map_err(|err| format!("{}", err))?
        },
        "name" => {
            transactions::table
                .filter(transactions::name.eq(&value))
                .order((transactions::time.desc(), transactions::id.desc()))
                .load::<Transaction>(&*connection)
                .map_err(|err| format!("{}", err))?
        },
        _ => return Err(format!("Unknown key '{}'", key))
    };

    let mut context = Context {
        total: Decimal::new(0, 0),
        input: Decimal::new(0, 0),
        output: Decimal::new(0, 0),
        transactions: Vec::new()
    };

    for transaction in transactions.iter() {
        if let Ok(value) = Decimal::from_str(&transaction.amount) {
            context.total += value;
        }
    }

    let mut current = context.total;
    for transaction in transactions {
        let mut next = current;
        if let Ok(value) = Decimal::from_str(&transaction.amount) {
            next -= value;
            if value.is_sign_positive() {
                context.input += value;
            }
            if value.is_sign_negative() {
                context.output += value;
            }
        }
        context.transactions.push(TransactionContext {
            transaction: transaction,
            total: current.round_dp(2),
        });
        current = next;
    }

    context.total = context.total.round_dp(2);
    context.input = context.input.round_dp(2);
    context.output = context.output.round_dp(2);

    Ok(Template::render("transaction", &context))
}
