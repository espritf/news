-- Statistics events for later analysis. Applied with `make clickhouse`.
create database if not exists news;

create table if not exists news.embedding_events
(
    kind          LowCardinality(String), -- 'completed' | 'failed'
    news_id       Int32,
    attempts      Int32,
    duration_ms   UInt32,
    chunk_count   UInt32,
    content_chars UInt32,
    error         Nullable(String),
    created_at    DateTime64(3) default now64()
)
engine = MergeTree
order by (kind, created_at);
