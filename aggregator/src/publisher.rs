use crate::schema::{channels, items};
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use crate::translator;
use serde_json;
use serde_json::Value;

#[derive(Serialize)]
pub struct News {
    sources: Vec<String>,
    title: String,
    pub_date: NaiveDateTime,
}

type Item = (i32, String, String, NaiveDateTime, String);

fn not_published(conn: &mut SqliteConnection) -> QueryResult<Vec<Item>> {
    items::table
        .inner_join(channels::table)
        .filter(items::published_id.is_null())
        .select((items::id, items::link, items::title, items::pub_date, channels::language))
        .load(conn)
}

// grab all unpublished news, translate them and publish
pub fn publish(conn: &mut SqliteConnection) -> Result<()> {

    let items = not_published(conn)?;

    for (id, link, title, pub_date, lang) in items {

        let title = if lang == "en" {
            tracing::info!("Skip translation for {}", title);
            title
        } else {
            translator::translate(&lang, "en", &title)?
        };

        let publication = News {
            sources: vec![link],
            title,
            pub_date,
        };
        
        // get nedpoint from env variable
        let endpoint = std::env::var("NEWS_API_ENDPOINT")?;
        let response = reqwest::blocking::Client::new()
            .post(endpoint)
            .json(&publication)
            .send()?;
        
        let json = response.json::<Value>()?;
        
        // get id from json
        let published_id = json
            .get("id").ok_or(anyhow::anyhow!("No id in response"))?
            .as_u64().ok_or(anyhow::anyhow!("Id is not a number"))?;
        
        tracing::info!("Item {} publishded with id: {}", &publication.title, &published_id);

        diesel::update(items::table.filter(items::id.eq(id)))
            .set(items::published_id.eq(published_id as i32)) // Why i32?
            .execute(conn)?;
    }

    Ok(())
}
