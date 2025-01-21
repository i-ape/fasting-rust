use crate::errors::FastingAppError;
use crate::models::{FastingEvent, FastingSession};
use crate::schema::fasting_sessions::dsl::{fasting_sessions, stop_time, start_time, user_id as session_user_id};
use crate::schema::fasting_events::dsl::{fasting_events, start_time as event_start_time, stop_time, user_id as event_user_id};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

const CHECKPOINTS: [i64; 9] = [4, 12, 14, 16, 18, 24, 36, 48, 72]; // Checkpoints in hours

/// Displays the user's fasting history from the database.
pub fn show_fasting_history(conn: &mut SqliteConnection, user_id: i32) {
    match fasting_sessions
        .filter(session_user_id.eq(user_id)) // Filter sessions by user_id
        .load::<FastingSession>(conn)
    {
        Ok(sessions) => {
            println!("Fasting History:");
            if sessions.is_empty() {
                println!("No fasting history found for user ID: {}", user_id);
                return;
            }
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
        Err(e) => eprintln!("Error fetching fasting history: {:?}", e),
    }
}

/// Retrieves the fasting sessions for a specific user.
pub fn get_fasting_sessions(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingSession>, FastingAppError> {
    fasting_sessions
        .filter(session_user_id.eq(user_id)) // Filter sessions by user_id
        .select(FastingSession::as_select()) // Match struct fields explicitly
        .load::<FastingSession>(conn)        // Load results into FastingSession struct
        .map_err(FastingAppError::DatabaseError)
}

/// Calculates the average fasting duration for a specific user.
pub fn calculate_average_fasting_duration(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<i64>, FastingAppError> {
    let events: Vec<FastingEvent> = fasting_events
        .filter(event_user_id.eq(user_id))
        .filter(stop_time.is_not_null())
        .load(conn)
        .map_err(FastingAppError::DatabaseError)?;

    if events.is_empty() {
        return Ok(None);
    }

    let total_duration: i64 = events
        .iter()
        .filter_map(|event| event.stop_time.map(|stop| stop.signed_duration_since(event.start_time).num_minutes()))
        .sum();

    let average_duration = total_duration / events.len() as i64;
    Ok(Some(average_duration))
}

/// Calculates the total fasting time for a specific user.
pub fn calculate_total_fasting_time(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<i64, FastingAppError> {
    let events: Vec<FastingEvent> = fasting_events
        .filter(event_user_id.eq(user_id))
        .filter(stop_time.is_not_null())
        .load(conn)
        .map_err(FastingAppError::DatabaseError)?;

    let total_duration: i64 = events
        .iter()
        .filter_map(|event| event.stop_time.map(|stop| stop.signed_duration_since(event.start_time).num_minutes()))
        .sum();

    Ok(total_duration)
}
