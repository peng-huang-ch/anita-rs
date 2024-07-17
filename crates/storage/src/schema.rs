// @generated automatically by Diesel CLI.

diesel::table! {
    keys (id) {
        id -> Int4,
        chain -> Varchar,
        secret -> Varchar,
        pubkey -> Varchar,
        address -> Varchar,
        suffix -> Varchar,
        used_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    keys,
    users,
);
