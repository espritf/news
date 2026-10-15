-- Durable queue of news items whose content still needs to be chunked and embedded.
-- A row is inserted alongside each news item and deleted once its chunks are stored.
create table embedding_jobs
(
    news_id      integer primary key references news (id) on delete cascade,
    attempts     integer     not null default 0,
    last_error   text,
    run_after    timestamptz not null default now(),
    locked_until timestamptz
);

create index embedding_jobs_run_after_idx on embedding_jobs (run_after);

-- Backfill: queue every existing article that never got its chunks stored.
insert into embedding_jobs (news_id)
select n.id
from news n
where not exists (select 1 from news_chunks c where c.news_id = n.id);
