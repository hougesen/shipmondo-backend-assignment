// @generated automatically by Diesel CLI.

diesel::table! {
    shipment_packages (package_id) {
        package_id -> Text,
        shipment_id -> Integer,
    }
}

diesel::table! {
    shipments (id) {
        id -> Integer,
        user_id -> Integer,
        price -> Text,
    }
}

diesel::table! {
    user_balances (rowid) {
        rowid -> Integer,
        user_id -> Integer,
        amount -> Float,
        currency_code -> Text,
        timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        production -> Bool,
        is_deleted -> Bool,
    }
}

diesel::joinable!(shipments -> users (user_id));
diesel::joinable!(user_balances -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(shipment_packages, shipments, user_balances, users,);
