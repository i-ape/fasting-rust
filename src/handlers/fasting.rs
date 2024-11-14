use crate::errors::FastingAppError;
use crate::models::{FastingEvent, NewFastingEvent};
use crate::schema::fasting_events::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Starts a new fasting event
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    event_start_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    if find_active_fasting_event(conn, user_id_input)?.is_some() {
        return Err(FastingAppError::ExistingSessionError);
    }

    let new_event = NewFastingEvent {
        user_id: user_id_input,
        start_time: event_start_time,
        stop_time: None,
    };

    diesel::insert_into(fasting_events)
        .values(&new_event)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)

/// Stops the active fasting event
pub fn stop_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    end_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    diesel::update(
        fasting_events
            .filter(user_id.eq(user_id_input))
            .filter(stop_time.is_null()),
    )
    .set(stop_time.eq(Some(end_time_input)))
    .execute(conn)
    .map_err(FastingAppError::DatabaseError)
    
}

/// Retrieves the current fasting status
pub fn get_current_fasting_status(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<(NaiveDateTime, i64)>, FastingAppError> {
    fasting_events
        .filter(user_id.eq(user_id_input))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)
}
