create table position_transactions (
    wallet_id text not null,
    account_id text not null,
    position_id text not null,
    id text not null,
    name text not null,
    time text not null,
    units text not null,
    price text not null,
    value text not null,
    primary key (wallet_id, account_id, id),
    foreign key (wallet_id) references wallets(id),
    foreign key (wallet_id, account_id) references accounts(wallet_id, id)
    foreign key (wallet_id, account_id, position_id) references positions(wallet_id, account_id, id)
);
