use actix_web::{error, Path, Responder, State};
use diesel::prelude::*;
use puccinia::database::models::{Wallet, Account, Position, PositionTransaction};
use puccinia::database::schema::{wallets, accounts, positions, position_transactions};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use super::AppState;

pub fn position(info: (Path<(String, String, String)>, State<Arc<AppState>>)) -> impl Responder {
    let connection = info.1.db.lock()
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;
    let path = info.0.into_inner();
    let wallet_id: String = path.0;
    let account_id: String = path.1;
    let id: String = path.2;

    #[derive(Serialize)]
    struct TransactionContext {
        transaction: PositionTransaction,
        total: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        account: Account,
        position: Position,
        total: Decimal,
        input: Decimal,
        output: Decimal,
        transactions: Vec<TransactionContext>,
    }

    let wallet = wallets::table
        .find(&wallet_id)
        .first::<Wallet>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let account = accounts::table
        .find((&wallet_id, &account_id))
        .first::<Account>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let position = positions::table
        .find((&wallet_id, &account_id, &id))
        .first::<Position>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let position_total = if let Ok(units) = Decimal::from_str(&position.units) {
        units.clone()
    } else {
        Decimal::new(0, 0)
    };

    let mut context = Context {
        wallet: wallet,
        account: account,
        position: position,
        total: position_total,
        input: Decimal::new(0, 0),
        output: Decimal::new(0, 0),
        transactions: Vec::new()
    };

    let transactions = position_transactions::table
        .filter(position_transactions::wallet_id.eq(&wallet_id))
        .filter(position_transactions::account_id.eq(&account_id))
        .filter(position_transactions::position_id.eq(&id))
        .order((position_transactions::time.desc(), position_transactions::id.desc()))
        .load::<PositionTransaction>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let mut current = context.total;
    for transaction in transactions {
        let mut next = current;
        if let Ok(units) = Decimal::from_str(&transaction.units) {
            next -= units;
            if units.is_sign_positive() {
                context.input += units;
            }
            if units.is_sign_negative() {
                context.output += units;
            }
        }
        context.transactions.push(TransactionContext {
            transaction: transaction,
            total: current,
        });
        current = next;
    }

    info.1.templates.render("position", &context)
}
