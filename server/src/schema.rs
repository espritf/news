// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    news (id) {
        id -> Int4,
        sources -> Json,
        title -> Text,
        pub_date -> Timestamp,
        content -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    news_chunks (id) {
        id -> Int4,
        news_id -> Int4,
        chunk_index -> Int4,
        chunk_text -> Text,
        chunk_v -> Vector,
    }
}

diesel::joinable!(news_chunks -> news (news_id));

diesel::allow_tables_to_appear_in_same_query!(news, news_chunks,);
