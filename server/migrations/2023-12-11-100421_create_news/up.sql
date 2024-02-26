create table news
(
    id          integer primary key not null ,
    sources     text_json not null ,
    title       text not null,
    pub_date    timestamp not null
);
