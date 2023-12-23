pub mod sources;

use super::schema::channels;
use super::schema::items;
use anyhow::Result;
use diesel::prelude::*;
use sources::{fetch, Config, Data, Item};

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
    published: bool,
    #[diesel(embed)]
    item: Item,
}

fn persist<'a>(conn: &mut SqliteConnection, data: Data) -> Result<()> {
    let id = diesel::insert_into(channels::table)
        .values(&data.channel)
        .on_conflict(channels::link)
        .do_update()
        .set((channels::last_build_date.eq(data.channel.last_build_date),))
        .returning(channels::id)
        .execute(conn)?;

    for item in data.items {
        diesel::insert_into(items::table)
            .values(ItemOfChannel {
                channel_id: id as i32,
                published: false,
                item: item.clone(),
            })
            .on_conflict_do_nothing()
            .execute(conn)?;
    }

    Ok(())
}
