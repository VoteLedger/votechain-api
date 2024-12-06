// @generated automatically by Diesel CLI.

diesel::table! {
    users (primary_account) {
        primary_account -> Text,
        refresh_token -> Text,
        last_login -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
