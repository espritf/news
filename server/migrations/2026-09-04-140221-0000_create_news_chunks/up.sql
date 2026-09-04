create table news_chunks
(
    id          serial primary key,
    news_id     integer not null references news (id) on delete cascade,
    chunk_index integer not null,
    chunk_text  text not null,
    chunk_v     vector(768) not null
);

create index news_chunks_news_id_idx on news_chunks (news_id);
