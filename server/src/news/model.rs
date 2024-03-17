use crate::schema::news;
use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::prelude::*;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::AsExpression;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub struct Sources(Vec<String>);

impl<DB> FromSql<Text, DB> for Sources
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&s)?))
    }
}

// https://docs.rs/diesel/2.1.4/diesel/serialize/trait.ToSql.html#
impl ToSql<Text, diesel::sqlite::Sqlite> for Sources
where
    String: ToSql<Text, diesel::sqlite::Sqlite>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        out.set_value(s);
        Ok(IsNull::No)
    }
}

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct News {
    id: i32,
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}

#[derive(Deserialize, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewsInput {
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}
