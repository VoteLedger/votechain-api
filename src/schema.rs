// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        accounts -> Array<Nullable<Text>>,
        signature -> Text,
        created_at -> Nullable<Timestamp>,
        refresh_token -> Text,
        last_login -> Nullable<Timestamp>,
    }
}
