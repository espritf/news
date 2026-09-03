pub mod sources;

mod content;

use super::schema::channels;
use super::schema::items;
use anyhow::Result;
use diesel::prelude::*;
use sources::{fetch, Config, Data, Item};
use std::collections::HashSet;

pub fn collect(conn: &mut SqliteConnection, sources: Vec<Config>) -> Result<()> {
    for source in sources {
        let _ = persist(conn, fetch(&source)?);
    }

    Ok(())
}

#[derive(Insertable)]
#[diesel(table_name = items)]
struct ItemOfChannel {
    channel_id: i32,
    content: String,
    #[diesel(embed)]
    item: Item,
}

fn persist(conn: &mut SqliteConnection, data: Data) -> Result<()> {
    let id = diesel::insert_into(channels::table)
        .values(&data.channel)
        .on_conflict(channels::link)
        .do_update()
        .set((channels::last_build_date.eq(data.channel.last_build_date),))
        .returning(channels::id)
        .get_result::<i32>(conn)?;

    let guids: Vec<&str> = data.items.iter().map(|item| item.guid.as_str()).collect();
    let existing: HashSet<String> = items::table
        .filter(items::guid.eq_any(guids))
        .select(items::guid)
        .load(conn)?
        .into_iter()
        .collect();

    // Only fetch the (expensive) article body for items we haven't seen before -
    // re-fetching content for already-known items on every scheduled run would be wasted work.
    let items: Vec<ItemOfChannel> = data
        .items
        .into_iter()
        .filter(|item| !existing.contains(&item.guid))
        .filter_map(|item| match content::fetch(&item.link) {
            Ok(content) => Some(ItemOfChannel {
                channel_id: id,
                content,
                item,
            }),
            Err(e) => {
                tracing::warn!("Skip item {} ({}): {}", item.guid, item.link, e);
                None
            }
        })
        .collect();

    let n = diesel::insert_or_ignore_into(items::table)
        .values(items)
        .execute(conn)?;

    tracing::info!("Persisted {} new items", n);

    Ok(())
}
