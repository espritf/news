use super::schema::channels;
use super::schema::items;
use crate::error::IsRequired;
use anyhow::Result;
use chrono::DateTime;
use chrono::NaiveDateTime;
use diesel::prelude::*;

pub fn collect(conn: &mut SqliteConnection, url: &String) -> Result<()> {
    let res = reqwest::blocking::get(url)?.text()?;
    let ch = rss::Channel::read_from(res.as_bytes())?;

    let (channel, items) = prepare(url, &ch)?;

    persist(conn, &channel, &items)
}

#[derive(Debug, Insertable)]
struct Channel<'a> {
    title: &'a str,
    link: &'a str,
    language: &'a str,
    last_build_date: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
struct Item<'a> {
    guid: &'a str,
    title: &'a str,
    link: &'a str,
    description: &'a str,
    pub_date: NaiveDateTime,
    tags: String,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
struct ItemOfChannel<'a> {
    channel_id: i32,
    #[diesel(embed)]
    item: Item<'a>
}

fn prepare<'a>(url: &'a str, ch: &'a rss::Channel) -> Result<(Channel<'a>, Vec<Item<'a>>)> {

    let last_build_date = DateTime::parse_from_rfc2822(ch.last_build_date().is_required()?)?;

    let channel = Channel {
        title: ch.title(),
        link: url,
        language: ch.language().is_required()?,
        last_build_date: last_build_date.naive_local(),
    };

    let items = ch
        .items()
        .iter()
        .map(|i| {
            let pub_date = DateTime::parse_from_rfc2822(i.pub_date().is_required()?)?;
            let tags: Vec<&str>  = i.categories().iter().map(|c| c.name()).collect();

            let item = Item {
                guid: i.guid().is_required()?.value(),
                title: i.title().is_required()?,
                link: i.link().is_required()?,
                description: i.description().is_required()?,
                pub_date: pub_date.naive_local(),
                tags: serde_json::to_string(&tags)?,
            };

            Ok(item)
        })
        .collect::<Result<Vec<Item>>>()?;

    Ok((channel, items))
}

fn persist(conn: &mut SqliteConnection, channel: &Channel, items: &[Item]) -> Result<()> {

    let id = diesel::insert_into(channels::table)
        .values(channel)
        .on_conflict(channels::link)
        .do_update()
        .set((channels::last_build_date.eq(channel.last_build_date),))
        .returning(channels::id)
        .execute(conn)?;

    for item in items {
        diesel::insert_into(items::table)
            .values(ItemOfChannel {
                channel_id: id as i32,
                item: item.clone(),
            })
            .on_conflict_do_nothing()
            .execute(conn)?;
    }

    Ok(())
}
