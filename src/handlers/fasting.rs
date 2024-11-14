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
    // Check if there's an active fasting event
    if find_active_fasting_event(conn, user_id)?.is_some() {
        return Err(FastingAppError::ExistingSessionError);
    }

    // Create and insert a new fasting event
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

/// Stops the active fasting event
pub fn stop_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    end_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    diesel::update(
        fasting_events
            .filter(user_id.eq(user_id))
            .filter(stop_time.is_null()),
    )
    .set(stop_time.eq(Some(end_time)))
    .execute(conn)
    .map_err(FastingAppError::DatabaseError)
}

/// Retrieves the current fasting status
pub fn get_current_fasting_status(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<(NaiveDateTime, i64)>, FastingAppError> {
    // Retrieve the active fasting event if any
    if let Some(event) = find_active_fasting_event(conn, user_id)? {
        let start_time = event.start_time;
        let duration = chrono::Utc::now().naive_utc()
            .signed_duration_since(start_time)
            .num_minutes();
        Ok(Some((start_time, duration)))
    } else {
        Ok(None)
    }
}

/// Helper function to find an active fasting event for a specific user
fn find_active_fasting_event(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<FastingEvent>, FastingAppError> {
    fasting_events
        .filter(user_id.eq(user_id))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)
}
