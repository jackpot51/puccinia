create table wallets (
    id text not null,
    name text not null,
    primary key (id)
);

create table accounts (
    wallet_id text not null,
    id text not null,
    name text not null,
    primary key (wallet_id, id),
    foreign key (wallet_id) references wallets(id)
);

create table positions (
    wallet_id text not null,
    account_id text not null,
    id text not null,
    name text not null,
    units text not null,
    price text not null,
    primary key (wallet_id, account_id, id),
    foreign key (wallet_id) references wallets(id),
    foreign key (wallet_id, account_id) references accounts(wallet_id, id)
);

create table transactions (
    wallet_id text not null,
    account_id text not null,
    id text not null,
    name text not null,
    time text not null,
    amount text not null,
    primary key (wallet_id, account_id, id),
    foreign key (wallet_id) references wallets(id),
    foreign key (wallet_id, account_id) references accounts(wallet_id, id)
);
