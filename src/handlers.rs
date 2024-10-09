use diesel::prelude::*;
use diesel::dsl::*;
use crate::models::{User, NewUser, FastingSession, NewFastingSession};
use crate::schema::{users, fasting_sessions};
use chrono::{NaiveDateTime, Utc};
use diesel::insert_into;
use bcrypt::{hash, verify};

// Function to start a fasting session
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<FastingSession, diesel::result::Error> {
    // Get the current time as NaiveDateTime for start time
    let start_time: NaiveDateTime = Utc::now().naive_utc();

    let new_session = NewFastingSession { user_id, start_time, end_time: None };
    insert_into(fasting_sessions::table)
        .values(&new_session)
        .execute(conn)?;

    fasting_sessions::table.order(fasting_sessions::id.desc()).first(conn)
}

// Function to stop a fasting session
pub fn stop_fasting(
    conn: &mut SqliteConnection,
    session_id: i32,
) -> Result<FastingSession, diesel::result::Error> {
    let end_time: NaiveDateTime = Utc::now().naive_utc();

    diesel::update(fasting_sessions::table.filter(fasting_sessions::id.eq(session_id)))
        .set(fasting_sessions::end_time.eq(Some(end_time)))
        .execute(conn)?;

    fasting_sessions::table.filter(fasting_sessions::id.eq(session_id)).first(conn)
}
