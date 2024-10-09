use crate::models::{FastingSession, NewFastingSession};
use crate::schema::fasting_sessions;
use chrono::{NaiveDateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

/// Creates a new fasting session for a given `user_id` with the current start time.
pub fn start_fasting(
    conn: &mut SqliteConnection, // Ensure this matches your `establish_connection` return type
    user_id: i32,
) -> Result<FastingSession, Error> {
    // Get the current time as `NaiveDateTime`
    let start_time: NaiveDateTime = Utc::now().naive_utc();

    // Create a new fasting session with `end_time` set to `None`
    let new_session = NewFastingSession {
        user_id,
        start_time,
        end_time: None, // `None` corresponds to a `Nullable<Timestamp>`
    };

    // Insert the new session into the `fasting_sessions` table
    insert_into(fasting_sessions::table)
        .values(&new_session)
        .execute(conn)?;

    // Retrieve and return the most recently inserted session
    fasting_sessions::table
        .order(fasting_sessions::id.desc())
        .first(conn)
}

/// Stops an existing fasting session by updating its `end_time` with the current time.
pub fn stop_fasting(conn: &mut SqliteConnection, session_id: i32) -> Result<FastingSession, Error> {
    // Get the current time as `NaiveDateTime` for the end time
    let end_time: NaiveDateTime = Utc::now().naive_utc();

    // Update the session with the given `session_id` to set its `end_time`
    diesel::update(fasting_sessions::table.filter(fasting_sessions::id.eq(session_id)))
        .set(fasting_sessions::end_time.eq(Some(end_time))) // Use `Some(end_time)` to indicate a `Nullable<Timestamp>`
        .execute(conn)?;

    // Retrieve and return the updated session
    fasting_sessions::table
        .filter(fasting_sessions::id.eq(session_id))
        .first(conn)
}

/// Retrieves all fasting sessions for a given `user_id`.
pub fn get_fasting_sessions(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingSession>, Error> {
    // Query all fasting sessions for the given `user_id`
    fasting_sessions::table
        .filter(fasting_sessions::user_id.eq(user_id))
        .load::<FastingSession>(conn)
}
