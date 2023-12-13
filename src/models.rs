use super::schema::channels;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = channels)]
pub struct NewChannel {
    pub title: String,
    pub link: String,
    pub language: String,
    pub last_build_date: NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Channel {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub language: String,
    pub last_build_date: NaiveDateTime,
}
