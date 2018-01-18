table! {
    accounts (wallet_id, id) {
        wallet_id -> Text,
        id -> Text,
        name -> Text,
    }
}

table! {
    positions (wallet_id, account_id, id) {
        wallet_id -> Text,
        account_id -> Text,
        id -> Text,
        name -> Text,
        units -> Text,
        price -> Text,
        value -> Text,
    }
}

table! {
    transactions (wallet_id, account_id, id) {
        wallet_id -> Text,
        account_id -> Text,
        id -> Text,
        name -> Text,
        time -> Text,
        amount -> Text,
    }
}

table! {
    wallets (id) {
        id -> Text,
        name -> Text,
    }
}

joinable!(accounts -> wallets (wallet_id));
joinable!(positions -> wallets (wallet_id));
joinable!(transactions -> wallets (wallet_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    positions,
    transactions,
    wallets,
);
