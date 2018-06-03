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
        total_units: Decimal,
        value: Decimal,
        value_change: Decimal,
        total_value: Decimal,
    }

    #[derive(Serialize)]
    struct Context {
        wallet: Wallet,
        account: Account,
        position: Position,
        original_units: Decimal,
        original_value: Decimal,
        input_units: Decimal,
        input_value: Decimal,
        output_units: Decimal,
        output_value: Decimal,
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

    let position_units = Decimal::from_str(&position.units).unwrap_or(Decimal::new(0, 0));
    let position_price = Decimal::from_str(&position.price).unwrap_or(Decimal::new(0, 0));
    let position_value = Decimal::from_str(&position.value).unwrap_or(Decimal::new(0, 0));

    let mut context = Context {
        wallet: wallet,
        account: account,
        position: position,
        original_units: Decimal::new(0, 0),
        original_value: Decimal::new(0, 0),
        input_units: Decimal::new(0, 0),
        input_value: Decimal::new(0, 0),
        output_units: Decimal::new(0, 0),
        output_value: Decimal::new(0, 0),
        transactions: Vec::new()
    };

    let transactions = position_transactions::table
        .filter(position_transactions::wallet_id.eq(&wallet_id))
        .filter(position_transactions::account_id.eq(&account_id))
        .filter(position_transactions::position_id.eq(&id))
        .order((position_transactions::time.desc(), position_transactions::id.desc()))
        .load::<PositionTransaction>(&*connection)
        .map_err(|err| error::ErrorInternalServerError(format!("{}", err)))?;

    let mut current_units = position_units;
    let mut current_value = position_value;
    for transaction in transactions {
        let mut next_units = current_units;
        let mut next_value = current_value;
        let mut value = Decimal::new(0, 0);
        if let Ok(units) = Decimal::from_str(&transaction.units) {
            value = units * position_price;
            next_units -= units;
            next_value -= value;
            if units.is_sign_positive() {
                context.input_units += units;
            }
            if units.is_sign_negative() {
                context.output_units += units;
            }
        }

        let mut value_change = Decimal::new(0, 0);
        if let Ok(transaction_value) = Decimal::from_str(&transaction.value) {
            value_change = value + transaction_value;
            if transaction_value.is_sign_positive() {
                context.input_value += transaction_value;
            }
            if transaction_value.is_sign_negative() {
                context.output_value += transaction_value;
            }
        }

        context.transactions.push(TransactionContext {
            transaction: transaction,
            total_units: current_units,
            value: value.round_dp(2),
            value_change: value_change.round_dp(2),
            total_value: current_value.round_dp(2),
        });

        current_units = next_units;
        current_value = next_value;
    }

    context.original_units = current_units;
    context.original_value = current_value.round_dp(2);
    context.input_value = context.input_value.round_dp(2);
    context.output_value = context.output_value.round_dp(2);

    info.1.templates.render("position", &context)
}
