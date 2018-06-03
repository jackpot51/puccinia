# puccinia
A self-hosted solution for accounting. Known to infect Mint with Rust.

## Introduction
Puccinia can access the following sources of financial data:
- Bank, credit card, and investment accounts using [OFX](http://ofx.net/)
  - [American Express](https://www.americanexpress.com/)
  - [Fidelity](https://www.fidelity.com/)
  - [Tangerine](https://www.tangerine.ca/)
  - [USAA](https://www.usaa.com/)
  - [Vanguard](https://www.vanguard.com/)
  - More can be added using http://ofxhome.com/ as a reference
- Cryptocurrency
  - Bitcoin exchange price using [coinnect](https://github.com/hugues31/coinnect)
  - Bitcoin address balances through [blockchain.info](https://blockchain.info/api)
- Manual entry

## Documentation

More documentation is available in the `doc` folder.

- [Database](./doc/database.md)
- [Features](./doc/features.md)

## Usage

You must have libsqlite3 installed to use `puccinia`. On Ubuntu, you may install
it with the following command:

```
sudo apt install libsqlite3-dev
```

Run the following commands to create the database.

```
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
```

Copy the `example.toml` to `secret.toml`, which is ignored by default. Modify
this file to include your own accounts.

Run `puccinia` with the path to your configuration to download your account
information:

```
cargo run --release secret.toml
```

Run `puccinia` without a path to simply used the cached information:

```
cargo run --release
```

## HTTPS

To enable HTTPS, generate a key with `openssl`. To adjust the expiration time,
add for example `-days 365`. To not require a password, add `-nodes`

```
openssl req -x509 -newkey rsa:4096 -keyout secret.key -out secret.crt
```

Run `puccinia` as above with an additional `--https` argument:

```
cargo run --release -- --https
```
