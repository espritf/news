create table channels
(
    id              integer primary key not null ,
    title           text not null,
    link            text not null,
    language        text not null ,
    last_build_date timestamp not null
);

create table tags
(
    id   integer primary key not null ,
    name text not null
);

create table news_tags
(
    id      integer primary key not null ,
    news_id integer not null ,
    tag_id  integer not null ,
    foreign key (news_id) references news (id) on delete cascade,
    foreign key (tag_id) references tags (id) on delete cascade
);

create table news
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
