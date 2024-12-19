use crate::errors::{handle_error, FastingAppError};
use crate::models::{FastingEvent, NewFastingEvent};
use crate::schema::fasting_events::dsl::{fasting_events, stop_time, user_id as schema_user_id};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Starts a new fasting event for a user.
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    event_start_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    if find_active_fasting_event(conn, user_id)?.is_some() {
        return Err(FastingAppError::ExistingSessionError);
    }

    let new_event = NewFastingEvent {
        user_id,
        start_time: event_start_time,
        stop_time: None,
    };

    diesel::insert_into(fasting_events)
        .values(&new_event)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Stops an active fasting event for a user.
pub fn stop_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    end_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    diesel::update(
        fasting_events
            .filter(schema_user_id.eq(user_id))
            .filter(stop_time.is_null()),
    )
    .set(stop_time.eq(Some(end_time)))
    .execute(conn)
    .map_err(FastingAppError::DatabaseError)
}

/// Retrieves the current fasting status for a user.
pub fn get_current_fasting_status(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<(NaiveDateTime, i64)>, FastingAppError> {
    if let Some(event) = find_active_fasting_event(conn, user_id)? {
        let start_time = event.start_time;
        let duration = Utc::now()
            .naive_utc()
            .signed_duration_since(start_time)
            .num_minutes();
        Ok(Some((start_time, duration)))
    } else {
        Ok(None)
    }
}

/// Finds an active fasting event for a user.
fn find_active_fasting_event(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<FastingEvent>, FastingAppError> {
    fasting_events
        .filter(schema_user_id.eq(user_id))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)
}
pub fn manage_fasting_session(conn: &mut diesel::SqliteConnection, user_id: i32) {
    match start_fasting(conn, user_id, chrono::Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session started."),
        Err(e) => handle_error(e),
    }

    match stop_fasting(conn, user_id, chrono::Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session stopped."),
        Err(e) => handle_error(e),
    }
}
