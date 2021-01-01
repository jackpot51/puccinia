create table transfers (
    from_wallet_id text not null,
    from_account_id text not null,
    from_id text not null,
    to_wallet_id text not null,
    to_account_id text not null,
    to_id text not null,
    primary key (from_wallet_id, from_account_id, from_id, to_wallet_id, to_account_id, to_id),
    foreign key (from_wallet_id) references wallets(id),
    foreign key (from_wallet_id, from_account_id) references accounts(wallet_id, id)
    foreign key (from_wallet_id, from_account_id, from_id) references transfers(wallet_id, account_id, id)
    foreign key (to_wallet_id) references wallets(id),
    foreign key (to_wallet_id, to_account_id) references accounts(wallet_id, id)
    foreign key (to_wallet_id, to_account_id, to_id) references transfers(wallet_id, account_id, id)
);
