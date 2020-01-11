use actix_web::{error, web::Path, Responder, web::Data};
use diesel::prelude::*;
use puccinia::database::models::Transaction;
use puccinia::database::schema::transactions;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use super::AppState;

pub fn transaction_all(state: Data<Arc<AppState>>) -> impl Responder {
    transaction_(state, String::new(), String::new())
}

pub fn transaction(info: (Path<(String, String)>, Data<Arc<AppState>>)) -> impl Responder {
    let path = info.0.into_inner();
    transaction_(info.1, path.0, path.1)
}

fn transaction_(state: Data<Arc<AppState>>, key: String, value: String) -> impl Responder {
    let connection = state.db.lock().map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    #[derive(Serialize)]
    struct TransactionContext {
        transaction: Transaction,
        total: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        original: Decimal,
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
                .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?
        },
        "time" => {
            transactions::table
                .filter(transactions::time.eq(&value))
                .order((transactions::time.desc(), transactions::id.desc()))
                .load::<Transaction>(&*connection)
                .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?
        },
        "name" => {
            transactions::table
                .filter(transactions::name.eq(&value))
                .order((transactions::time.desc(), transactions::id.desc()))
                .load::<Transaction>(&*connection)
                .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?
        },
        _ => return Err(error::ErrorInternalServerError(format!("Unknown key '{}'", key)))
    };

    let mut context = Context {
        original: Decimal::new(0, 0),
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

    context.original = current.round_dp(2);
    context.total = context.total.round_dp(2);
    context.input = context.input.round_dp(2);
    context.output = context.output.round_dp(2);

    state.templates.render("transaction", &context)
}
