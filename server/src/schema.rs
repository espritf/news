// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    news (id) {
        id -> Int4,
        sources -> Json,
        title -> Text,
        pub_date -> Timestamp,
        title_v -> Vector,
    }
}
