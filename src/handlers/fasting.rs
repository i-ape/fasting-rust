use crate::errors::FastingAppError;
use crate::models::{FastingEvent, FastingSession};
use crate::schema::fasting_events::{self, stop_time};
use crate::schema::fasting_events::dsl::{id as event_id, goal_id as event_goal_id};
use crate::schema::fasting_sessions::dsl::{fasting_sessions, user_id as session_user_id};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

///
/// /// ✅ Starts fasting, with or without a goal.
///
/// - **If a goal exists**, it links the fast to the goal.
/// - **If no goal is provided**, it just starts fasting normally.
/// - **Ensures no active fasting session already exists.**
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id: i32,
    event_start_time: NaiveDateTime,
    goal_id: Option<i32>, // ✅ New parameter for fasting goal
) -> Result<(), FastingAppError> {
    use crate::models::NewFastingEvent;

    let new_event = NewFastingEvent {
        user_id,
        start_time: event_start_time,
        stop_time: None,
        created_at: Some(Utc::now().naive_utc()),
        goal_id, // ✅ Store goal_id (if provided)
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

    update(fasting_events::find(ongoing_event.id))
        .set(stop_time.eq(&Some(event_end_time)))
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

/// ✅ Updates the fasting goal **without resetting the fast timer**.
pub fn update_fasting_goal(
    conn: &mut SqliteConnection,
    user_id_input: i32,
    new_goal_id: Option<i32>,
) -> Result<(), FastingAppError> {
    let active_fast = fasting_events::filter(schema_user_id.eq(user_id_input))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?;

    if let Some(fast) = active_fast {
        diesel::update(fasting_events.filter(event_id.eq(fast.id))) // ✅ Use `event_id`
            .set(event_goal_id.eq(new_goal_id)) // ✅ Use `event_goal_id`
            .execute(conn)
            .map_err(FastingAppError::DatabaseError)?;

        println!(
            "Updated fasting goal for user {}. New goal ID: {:?}",
            user_id_input, new_goal_id
        );
        Ok(())
    } else {
        Err(FastingAppError::SessionError(
            "No active fasting session found.".to_string(),
        ))
    }
}

/// ✅ Removes the fasting goal **without stopping the fast**.
pub fn remove_fasting_goal(
    conn: &mut SqliteConnection,
    user_id_input: i32,
) -> Result<(), FastingAppError> {
    update_fasting_goal(conn, user_id_input, None)
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
