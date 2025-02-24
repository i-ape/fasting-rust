use crate::errors::FastingAppError;
use crate::models::{FastingEvent, FastingSession};
use crate::schema::fasting_events::dsl::{
    fasting_events, stop_time as event_stop_time, user_id as event_user_id,
};
use crate::schema::fasting_sessions::dsl::{fasting_sessions, user_id as session_user_id};
use chrono::Utc;
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Retrieves and displays the user's fasting history.
pub fn show_fasting_history(conn: &mut SqliteConnection, user_id: i32) {
    match get_fasting_sessions(conn, user_id) {
        Ok(sessions) => {
            println!("Fasting History:");
            if sessions.is_empty() {
                println!("No fasting history found for user ID: {}", user_id);
            } else {
                for session in sessions {
                    let end_time = session.stop_time.unwrap_or_else(|| Utc::now().naive_utc());
                    let duration = end_time - session.start_time;
                    println!(
                        "- Start: {}, End: {}, Duration: {} minutes",
                        session.start_time,
                        session.stop_time
                            .map_or_else(|| "Ongoing".to_string(), |end| end.to_string()),
                        duration.num_minutes()
                    );
                }
            }
        }
        Err(e) => eprintln!("Error fetching fasting history: {:?}", e),
    }
}

/// Retrieves fasting sessions for a specific user.
fn get_fasting_sessions(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingSession>, FastingAppError> {
    fasting_sessions
        .filter(session_user_id.eq(user_id))
        .select(FastingSession::as_select())
        .load::<FastingSession>(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Calculates the average fasting duration for a specific user.
pub fn calculate_average_fasting_duration(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<i64>, FastingAppError> {
    let events = get_fasting_events_with_end_time(conn, user_id)?;

    if events.is_empty() {
        return Ok(None);
    }

    let total_duration: i64 = events
        .iter()
        .filter_map(|event| {
            event
                .stop_time
                .map(|stop| stop.signed_duration_since(event.start_time).num_minutes())
        })
        .sum();

    let event_count = events.len() as i64;

    if event_count == 0 {
        return Ok(None); // Avoid division by zero
    }

    let average_duration = total_duration / event_count;
    Ok(Some(average_duration))
}

/// Calculates the total fasting time for a specific user.
pub fn calculate_total_fasting_time(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<i64, FastingAppError> {
    let events = get_fasting_events_with_end_time(conn, user_id)?;

    let total_duration: i64 = events
        .iter()
        .filter_map(|event| {
            event
                .stop_time
                .map(|stop| stop.signed_duration_since(event.start_time).num_minutes())
        })
        .sum();

    Ok(total_duration)
}

/// Retrieves fasting events with a valid `stop_time`.
fn get_fasting_events_with_end_time(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingEvent>, FastingAppError> {
    fasting_events
        .filter(event_user_id.eq(user_id))
        .filter(event_stop_time.is_not_null())
        .select(FastingEvent::as_select()) // Explicitly map fields to struct
        .load::<FastingEvent>(conn)
        .map_err(FastingAppError::DatabaseError)
}
