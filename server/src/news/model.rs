use crate::schema::news;
use chrono::NaiveDateTime;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::prelude::*;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::AsExpression;
use serde::{Deserialize, Serialize};

type Backend = diesel::pg::Pg;
//use diesel::backend::Backend;

#[derive(Serialize, Deserialize, Debug, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub struct Sources(Vec<String>);

impl<DB> FromSql<Text, DB> for Sources
where
    DB: diesel::backend::Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        Ok(Self(serde_json::from_str(&s)?))
    }
}

// https://docs.rs/diesel/2.1.4/diesel/serialize/trait.ToSql.html#
impl ToSql<Text, Backend> for Sources
where
    String: ToSql<Text, Backend>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, Backend>,
    ) -> diesel::serialize::Result {
        let s = serde_json::to_string(&self.0)?;
        //s.to_sql(out)
        <String as ToSql<Text, Backend>>::to_sql(&s, &mut out.reborrow())
    }
}

#[derive(Serialize, Queryable, Selectable, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct News {
    id: i32,
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}

impl News {
    pub fn new(id: i32, title: String, pub_date: NaiveDateTime, sources: Vec<String>) -> Self {
        Self {
            id,
            title,
            pub_date,
            sources: Sources(sources),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Insertable)]
#[diesel(table_name = news)]
#[diesel(check_for_backend(Backend))]
pub struct NewsInput {
    title: String,
    pub_date: NaiveDateTime,
    sources: Sources,
}
