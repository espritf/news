// @generated automatically by Diesel CLI.

diesel::table! {
    news (id) {
        id -> Integer,
        sources -> Text,
        title -> Text,
        pub_date -> Timestamp,
    }
}
