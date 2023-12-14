use diesel::prelude::*;
use crate::models::input::Channel;

pub fn add_channel(conn: &mut SqliteConnection, channel: Channel) {
    use crate::schema::channels;

    diesel::insert_into(channels::table)
        .values(&channel)
        .on_conflict(channels::link)
        .do_update()
        .set(&channel)
        .execute(conn)
        .expect("Error saving new channel");
}
