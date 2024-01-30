// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Integer,
        title -> Text,
        link -> Text,
        language -> Text,
        last_build_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        channel_id -> Integer,
        guid -> Text,
        title -> Text,
        link -> Text,
        tags -> Nullable<Text>,
        pub_date -> Timestamp,
    }
}

diesel::table! {
    news (id) {
        id -> Integer,
        source_id -> Integer,
        title -> Text,
        pub_date -> Timestamp,
    }
}

diesel::joinable!(items -> channels (channel_id));
diesel::joinable!(news -> items (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    items,
    news,
);
