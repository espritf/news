alter table news_chunks drop column chunk_v;
alter table news_chunks add column chunk_v vector(768) not null;
