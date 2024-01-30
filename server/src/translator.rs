use std::time::Duration;

use crate::schema::{channels, items, news};
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Insertable)]
#[diesel(table_name = news)]
pub struct News {
    source_id: i32,
    title: String,
    pub_date: NaiveDateTime,
}

#[derive(Deserialize)]
struct Response {
    response: String,
}

fn translate(text: &str) -> Result<String> {
    tracing::info!("Translate text: {}", text);

    let json = &serde_json::json!({
        "model": "translator",
        "stream": false,
        "prompt": text,
    });

    let res = reqwest::blocking::Client::builder()
        .timeout(Some(Duration::new(240, 0)))
        .build()?
        .post("http://localhost:11434/api/generate")
        .json(json)
        .send()?
        .json::<Response>()?;

    Ok(res.response.trim().to_owned())
}

// grab all unpublished news, translate them and publish
pub fn publish(conn: &mut SqliteConnection) -> Result<()> {
    type Item = (i32, String, NaiveDateTime, String);

    let items: Vec<Item> = items::table
        .inner_join(channels::table)
        .left_join(news::table)
        .filter(news::id.is_null())
        .select((items::id, items::title, items::pub_date, channels::language))
        .load(conn)?;

    for (id, title, pub_date, lang) in items {

        let title = if lang == "en" {
            tracing::info!("Skip translation for {}", title);
            title
        } else {
            translate(&title)?
        };

        let news = News {
            source_id: id,
            title,
            pub_date,
        };

        conn.transaction(|c| -> Result<()> {
            tracing::info!("Publish news with title: {}", news.title);

            diesel::insert_into(news::table).values(&news).execute(c)?;

            Ok(())
        })?;
    }

    Ok(())
}
