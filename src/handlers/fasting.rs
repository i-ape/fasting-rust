use crate::errors::FastingAppError;
use crate::models::{FastingEvent, FastingSession};
use crate::schema::fasting_events::dsl::{
    fasting_events, start_time, stop_time, user_id as schema_user_id,
};
use crate::schema::fasting_sessions::dsl::{fasting_sessions};
use chrono::Utc;
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Starts a fasting session for a user.
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    event_start_time: chrono::NaiveDateTime,
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
    event_end_time: chrono::NaiveDateTime,
) -> Result<(), FastingAppError> {
    use diesel::dsl::update;

    let ongoing_event = fasting_events
        .filter(schema_user_id.eq(user_id))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?;

    if let Some(event) = ongoing_event {
        update(fasting_events.find(event.id.unwrap()))
            .set(stop_time.eq(Some(event_end_time)))
            .execute(conn)
            .map(|_| ())
            .map_err(FastingAppError::DatabaseError)
    } else {
        Err(FastingAppError::Custom(
            "No ongoing fasting session found.".to_string(),
        ))
    }
}

/// Retrieves the current fasting status for a user.
pub fn get_current_fasting_status(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<(chrono::NaiveDateTime, i64)>, FastingAppError> {
    let ongoing_event = fasting_events
        .filter(schema_user_id.eq(user_id))
        .filter(stop_time.is_null()) // Find ongoing fasting event
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?;

    if let Some(event) = ongoing_event {
        let duration = Utc::now().naive_utc() - event.start_time;
        Ok(Some((event.start_time, duration.num_minutes())))
    } else {
        Ok(None) // No ongoing session
    }
}

/// Retrieves all fasting sessions for a user.
pub fn get_user_fasting_sessions(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingSession>, FastingAppError> {
    fasting_sessions
        .filter(schema_user_id.eq(user_id))
        .select(FastingSession::as_select())
        .load::<FastingSession>(conn)
        .map_err(FastingAppError::DatabaseError)
}
