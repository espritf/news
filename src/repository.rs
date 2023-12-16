use crate::models::Channel;
use diesel::prelude::*;

pub fn add_channel(conn: &mut SqliteConnection, channel: Channel) {
    use crate::schema::channels;
    use crate::schema::items;

    let id = diesel::insert_into(channels::table)
        .values((
            channels::title.eq(channel.title()),
            channels::link.eq(channel.link()),
            channels::language.eq(channel.language()),
            channels::last_build_date.eq(channel.last_build_date()),
        ))
        .on_conflict(channels::link)
        .do_update()
        .set((channels::last_build_date.eq(channel.last_build_date()),))
        .returning(channels::id)
        .execute(conn)
        .expect("Error saving new channel");

    for item in channel.items() {
        diesel::insert_into(items::table)
            .values((
                items::channel_id.eq(id as i32),
                items::guid.eq(item.guid()),
                items::title.eq(item.title()),
                items::link.eq(item.link()),
                items::description.eq(item.description()),
                items::pub_date.eq(item.pub_date()),
            ))
            .on_conflict_do_nothing()
            .execute(conn)
            .expect("Error saving new item");
    }
}
