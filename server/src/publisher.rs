use crate::schema::{channels, items, news};
use anyhow::Result;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::translator;
use serde_json;
use serde_json::json;

#[derive(Insertable)]
#[diesel(table_name = news)]
pub struct News {
    sources: String,
    title: String,
    pub_date: NaiveDateTime,
}

// grab all unpublished news, translate them and publish
pub fn publish(conn: &mut SqliteConnection) -> Result<()> {
    type Item = (i32, String, String, NaiveDateTime, String);

    let items: Vec<Item> = items::table
        .inner_join(channels::table)
        .filter(items::published_id.is_null())
        .select((items::id, items::link, items::title, items::pub_date, channels::language))
        .load(conn)?;

    for (id, link, title, pub_date, lang) in items {

        let title = if lang == "en" {
            tracing::info!("Skip translation for {}", title);
            title
        } else {
            translator::translate(&lang, "en", &title)?
        };

        let publication = News {
            sources: json!([link]).to_string(),
            title,
            pub_date,
        };

        conn.transaction(|c| -> Result<()> {
            tracing::info!("Publish news with title: {}", publication.title);

            let published_id = diesel::insert_into(news::table)
                .values(&publication)
                .returning(news::id)
                .get_result::<i32>(c)?;
            
            diesel::update(items::table.filter(items::id.eq(id)))
                .set(items::published_id.eq(published_id))
                .execute(c)?;

            Ok(())
        })?;
    }

    Ok(())
}
