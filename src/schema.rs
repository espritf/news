// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Integer,
        title -> Text,
        link -> Text,
        language -> Text,
        last_build_date -> Timestamp,
    }
}

diesel::table! {
    news (id) {
        id -> Integer,
        channel_id -> Integer,
        guid -> Text,
        title -> Text,
        link -> Text,
        description -> Text,
        pub_date -> Timestamp,
    }
}

diesel::table! {
    news_tags (id) {
        id -> Integer,
        news_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(news -> channels (channel_id));
diesel::joinable!(news_tags -> news (news_id));
diesel::joinable!(news_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    channels,
    news,
    news_tags,
    tags,
);
