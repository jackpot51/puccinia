## Database Tables

The database is composed of the `wallets`, `accounts`, `positions`, and
`transactions` tables. This file documents each table, their fields, and their
interactions.

### Wallets

Each row in the `wallets` table identifies a wallet. A wallet is a set of
`accounts`, for example from a single bank, that are organized together.

#### Fields

- `id` - A unique identifier for the wallet. Should not contain
spaces or special characters
- `name` - A name for the wallet in user interfaces.

#### Example

| id       | name                 |
|----------|----------------------|
| vanguard | Vanguard Investments |

### Accounts

Each row in the `accounts` table identifies an account. An account is a
collection of `positions` and `transactions`, for example, a bank account.

#### Fields

- `wallet_id` - The `id` of the row in the `wallets` table identifying the
wallet this account belongs to.
- `id` - A unique identifier for the account within a wallet. Typically the
account number.
- `name` - A name for the account in user interfaces. Typically the account
type followed by the account number

#### Example

| wallet_id | id       | name                |
|-----------|----------|---------------------|
| vanguard  | 12345678 | INVESTMENT_12345678 |

### Positions

Each row in the `positions` table identifies a position. A position is an amount
of some asset in `units`, for example a stock, that has a `price` and a total
`value`. Cash accounts, such as bank accounts, have a single position with a
unit `price` of `1`.

#### Fields

- `wallet_id` - The `id` of the row in the `wallets` table identifying the
wallet this position belongs to.
- `account_id` - The `id` of the row in the `accounts` table identifying the
account this position belongs to.
- `id` - A unique identifier for the position within an account. Typically the
stock ticker.
- `name` - A name for the position in user interfaces. Typically the name of the
stock.
- `units` - The number of units in the position. Typically the number of shares.
This should not be rounded.
- `price` - The price of each unit in the position, in the user's currency.
Typically the price per share. This should not be rounded.
- `value` - The total value of this position, in the user's currency. This
should be equal to `units` multiplied by `price`. This should not be rounded.

#### Example

| wallet_id | account_id | id       | name          | units  | price | value     |
|-----------|------------|----------|---------------|--------|-------|-----------|
| vanguard  | 12345678   | PUCCINIA | Puccinia Fund | 123.45 | 67.89 | 8381.0205 |


### Transactions

Each row in the `transactions` table identifies a transaction. A transaction is
an amount change of value of an account, at a certain time.

#### Fields

- `wallet_id` - The `id` of the row in the `wallets` table identifying the
wallet this transaction belongs to.
- `account_id` - The `id` of the row in the `accounts` table identifying the
account this transaction belongs to.
- `id` - A unique identifier for the transaction within an account. Typically
generated by the source of the transaction data.
- `name` - A name for the transaction in user interfaces. Typically identifies
the organization that created a transaction.
- `time` - The date and optionally time of the transaction, in
`yyyy-MM-dd hh:mm:ss` format, and in the user's timezone. Typically is only
accurate to the day, due to the source of the transaction data.
- `amount` - The amount of the transaction, in the user's currency. Should be
negative to indicate money leaving the account, and positive to indicate money
entering the account. This should not be rounded.

#### Example

| wallet_id | account_id | id       | name              | units      | amount  |
|-----------|------------|----------|-------------------|------------|---------|
| vanguard  | 12345678   | 87654321 | Puccinia Fund Buy | 2018-05-31 | 8000.00 |
