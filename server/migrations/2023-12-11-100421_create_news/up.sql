create table channels
(
    id              integer primary key not null ,
    title           text not null,
    link            text not null unique,
    language        text not null ,
    last_build_date timestamp
);

create table items
(
    id          integer primary key not null ,
    channel_id  integer not null ,
    guid        text unique not null ,
    title       text not null,
    link        text not null,
    tags        text_json,
    pub_date    timestamp not null ,
    foreign key (channel_id) references channels (id) on delete cascade
);

create table news
(
    id          integer primary key not null ,
    source_id   integer not null ,
    title       text not null,
    pub_date    timestamp not null ,
    foreign key (source_id) references items (id) on delete cascade
);
