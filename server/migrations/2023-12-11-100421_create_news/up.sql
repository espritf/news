create table news
(
    id          serial primary key,
    sources     json not null ,
    title       text not null,
    pub_date    timestamp not null
);
