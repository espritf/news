alter table news add column content text not null default '';
alter table news alter column content drop default;
