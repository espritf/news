create table channels
(
    id              integer primary key not null ,
    title           text not null,
    link            text not null unique,
    language        text not null ,
    last_build_date timestamp not null
);

create table items
(
    id          integer primary key not null ,
    channel_id  integer not null ,
    guid        text unique not null ,
    title       text not null,
    link        text not null,
    description text not null,
    pub_date    timestamp not null ,
    foreign key (channel_id) references channels (id) on delete cascade
);
