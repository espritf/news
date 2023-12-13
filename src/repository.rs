use diesel::prelude::*;
use crate::models::NewChannel;

pub fn add_channel(conn: &mut SqliteConnection, channel: NewChannel) {
    use crate::schema::channels;

    diesel::insert_or_ignore_into(channels::table)
        .values(&channel)
        .on_conflict_do_nothing()
        .execute(conn)
        .expect("Error saving new channel");
}
