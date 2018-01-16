table! {
    positions (id) {
        id -> Integer,
        name -> Text,
        units -> Text,
        price -> Text,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        name -> Text,
        amount -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    positions,
    transactions,
);
