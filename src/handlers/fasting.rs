use crate::errors::FastingAppError;
use crate::models::{FastingEvent, FastingSession};
use crate::schema::fasting_events::dsl::{
    fasting_events, stop_time, user_id as schema_user_id,
};
use crate::schema::fasting_sessions::dsl::{fasting_sessions, user_id as session_user_id};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Starts a fasting session for a user.
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    event_start_time: NaiveDateTime,
) -> Result<(), FastingAppError> {
    use crate::models::NewFastingEvent;

    let new_event = NewFastingEvent {
        user_id,
        start_time: event_start_time,
        stop_time: None,
        created_at: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(fasting_events)
        .values(&new_event)
        .execute(conn)
        .map(|_| ())
        .map_err(FastingAppError::DatabaseError)
}

/// Stops a fasting session for a user.
pub fn stop_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    event_end_time: NaiveDateTime,
) -> Result<(), FastingAppError> {
    use diesel::dsl::update;

    let ongoing_event = find_ongoing_fasting_event(conn, user_id)?;

    update(fasting_events.find(ongoing_event.id))
        .set(stop_time.eq(Some(event_end_time)))
        .execute(conn)
        .map(|_| ())
        .map_err(FastingAppError::DatabaseError)
}

/// Retrieves the current fasting status for a user.
pub fn get_current_fasting_status(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<(NaiveDateTime, i64)>, FastingAppError> {
    let ongoing_event = find_ongoing_fasting_event(conn, user_id)?;

    let duration = Utc::now().naive_utc() - ongoing_event.start_time;
    Ok(Some((ongoing_event.start_time, duration.num_minutes())))
}

/// Retrieves all fasting sessions for a user.
pub fn get_user_fasting_sessions(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingSession>, FastingAppError> {
    fasting_sessions
        .filter(session_user_id.eq(user_id))
        .select(FastingSession::as_select())
        .load::<FastingSession>(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Finds an ongoing fasting event for a user.
fn find_ongoing_fasting_event(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<FastingEvent, FastingAppError> {
    fasting_events
        .filter(schema_user_id.eq(user_id))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .map_err(|_| FastingAppError::SessionError("No ongoing fasting session found.".to_string()))
}
