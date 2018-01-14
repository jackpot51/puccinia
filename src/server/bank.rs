use decimal::d128;
use puccinia::Puccinia;
use rocket::State;
use rocket_contrib::Template;
use std::str::FromStr;

#[get("/bank/<id>")]
pub fn index(puccinia: State<Puccinia>, id: String) -> Result<Template, String> {
    #[derive(Serialize)]
    struct Account {
        id: String,
        kind: String,
        //amount: String,
    }

    #[derive(Serialize)]
    struct Context {
        id: String,
        name: String,
        accounts: Vec<Account>,
    }

    let bank = puccinia.bank.get(&id).ok_or(format!("bank '{}' not found", id))?;

    let mut context = Context {
        id: id,
        name: bank.name().to_string(),
        accounts: Vec::new()
    };

    for account in bank.accounts()? {
        //let amount = bank.amount(&account)?;
        context.accounts.push(Account {
            id: account.id,
            kind: account.kind,
            //amount: format!("{}", amount)
        });
    }

    Ok(Template::render("bank_index", &context))
}

#[get("/bank_account/<id>/<account_id>/<account_kind>")]
pub fn account(puccinia: State<Puccinia>, id: String, account_id: String, account_kind: String) -> Result<Template, String> {
    #[derive(Serialize)]
    struct Position {
        ticker: String,
        name: String,
        units: String,
        unit_price: String,
        market_value: String,
    }

    #[derive(Serialize)]
    struct Transaction {
        time: String,
        name: String,
        kind: String,
        amount: String,
    }

    #[derive(Serialize)]
    struct Context {
        bank_id: String,
        bank_name: String,
        id: String,
        kind: String,
        amount: String,
        positions: Vec<Position>,
        transactions: Vec<Transaction>
    }

    let bank = puccinia.bank.get(&id).ok_or(format!("bank '{}' not found", id))?;

    let mut context = Context {
        bank_id: id,
        bank_name: bank.name().to_string(),
        id: account_id.clone(),
        kind: account_kind.clone(),
        amount: String::new(),
        positions: Vec::new(),
        transactions: Vec::new()
    };

    if let Some(bank_ofx) = bank.as_ofx() {
        let response = bank_ofx.ofx(&account_id, &account_kind, None, None)?;

        let mut total = d128::zero();

        if let Some(balance) = response.balance {
            if let Some(amount) = balance.amount {
                total += d128::from_str(&amount).map_err(|_err| {
                    format!("invalid decimal: {}", amount)
                })?;
            }
        }

        for position in response.positions {
            if let Some(ref market_value) = position.market_value {
                total += d128::from_str(&market_value).map_err(|_err| {
                    format!("invalid decimal: {}", market_value)
                })?;
            }

            let mut name = None;
            let mut ticker = None;
            if let Some(ref p_id) = position.id {
                for security in &response.securities {
                    if let Some(ref s_id) = security.id {
                        if p_id == s_id {
                            name = security.name.clone();
                            ticker = security.ticker.clone();
                        }
                    }
                }
            }

            context.positions.push(Position {
                ticker: ticker.unwrap_or(String::new()),
                name: name.unwrap_or(String::new()),
                units: position.units.unwrap_or(String::new()),
                unit_price: position.unit_price.unwrap_or(String::new()),
                market_value: position.market_value.unwrap_or(String::new()),
            });
        }

        context.positions.sort_by(|a, b| {
            a.ticker.cmp(&b.ticker)
        });

        for transaction in response.transactions {
            context.transactions.push(Transaction {
                time: transaction.time.unwrap_or(String::new()),
                name: transaction.name.unwrap_or(String::new()),
                kind: transaction.kind.unwrap_or(String::new()),
                amount: transaction.amount.unwrap_or(String::new()),
            });
        }

        context.transactions.sort_by(|a, b| {
            a.time.cmp(&b.time)
        });

        context.amount = format!("{}", total);
    }

    Ok(Template::render("bank_account", &context))
}
