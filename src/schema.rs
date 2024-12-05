// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        primary_account -> Text,
        refresh_token -> Text,
        last_login -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
