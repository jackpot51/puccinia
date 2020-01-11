use actix_web::{error, web::Path, Responder, web::Data};
use diesel::prelude::*;
use puccinia::database::models::{Wallet, Account, Position, Transaction};
use puccinia::database::schema::{wallets, accounts, positions, transactions};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use super::AppState;

pub fn account(info: (Path<(String, String)>, Data<Arc<AppState>>)) -> impl Responder {
    let connection = info.1.db.lock()
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;
    let path = info.0.into_inner();
    let wallet_id: String = path.0;
    let id: String = path.1;

    #[derive(Serialize)]
    struct TransactionContext {
        transaction: Transaction,
        total: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        account: Account,
        original: Decimal,
        total: Decimal,
        input: Decimal,
        output: Decimal,
        positions: Vec<Position>,
        transactions: Vec<TransactionContext>,
    }

    let wallet = wallets::table
        .find(&wallet_id)
        .first::<Wallet>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let account = accounts::table
        .find((&wallet_id, &id))
        .first::<Account>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let mut context = Context {
        wallet: wallet,
        account: account,
        original: Decimal::new(0, 0),
        total: Decimal::new(0, 0),
        input: Decimal::new(0, 0),
        output: Decimal::new(0, 0),
        positions: Vec::new(),
        transactions: Vec::new()
    };

    let positions = positions::table
        .filter(positions::wallet_id.eq(&wallet_id))
        .filter(positions::account_id.eq(&id))
        .order(positions::id.asc())
        .load::<Position>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    for position in positions {
        if let Ok(value) = Decimal::from_str(&position.value) {
            context.total += value;
        }
        context.positions.push(position);
    }

    let transactions = transactions::table
        .filter(transactions::wallet_id.eq(&wallet_id))
        .filter(transactions::account_id.eq(&id))
        .order((transactions::time.desc(), transactions::id.desc()))
        .load::<Transaction>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

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

    info.1.templates.render("account", &context)
}
