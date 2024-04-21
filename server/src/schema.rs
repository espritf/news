// @generated automatically by Diesel CLI.

diesel::table! {
    news (id) {
        id -> Int4,
        sources -> Json,
        title -> Text,
        pub_date -> Timestamp,
    }
}
