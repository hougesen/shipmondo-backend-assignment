// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        production -> Bool,
        is_deleted -> Bool,
    }
}
