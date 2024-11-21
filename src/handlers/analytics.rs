use crate::errors::FastingAppError;
use crate::models::FastingEvent;
use crate::schema::fasting_events::dsl::{fasting_events, start_time, user_id as schema_user_id};
use crate::schema::fasting_events::stop_time;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Retrieves the complete fasting history for a specific user
pub fn get_fasting_history(
    conn: &mut SqliteConnection,
    user_id: i32, // This is the function parameter
) -> Result<Vec<FastingEvent>, FastingAppError> {
    fasting_events
        .filter(schema_user_id.eq(user_id)) // Using `schema_user_id` for the column
        .order(start_time.desc())
        .load::<FastingEvent>(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Calculates the average fasting duration for a specific user
pub fn calculate_average_fasting_duration(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Option<i64>, FastingAppError> {
    let events: Vec<FastingEvent> = fasting_events
        .filter(schema_user_id.eq(user_id)) // Using `schema_user_id` for the column
        .filter(stop_time.is_not_null())
        .load(conn)
        .map_err(FastingAppError::DatabaseError)?;

    if events.is_empty() {
        return Ok(None);
    }

    let total_duration: i64 = events
        .iter()
        .map(|event| {
            
            event
                .stop_time
                .unwrap()
                .signed_duration_since(event.start_time)
                .num_minutes()
        })
        .sum();

    let average_duration = total_duration / events.len() as i64;
    Ok(Some(average_duration))
}

/// Generates a weekly summary of fasting hours for a specific user within a date range
pub fn calculate_weekly_fasting_summary(
    conn: &mut SqliteConnection,
    user_id: i32,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<i64, FastingAppError> {
    let events: Vec<FastingEvent> = fasting_events
        .filter(schema_user_id.eq(user_id)) // Using `schema_user_id` for the column
        .filter(start_time.ge(start_date))
        .filter(stop_time.le(Some(end_date)))
        .load(conn)
        .map_err(FastingAppError::DatabaseError)?;

    let total_duration: i64 = events
        .iter()
        .map(|event| {
            
            event
                .stop_time
                .unwrap()
                .signed_duration_since(event.start_time)
                .num_minutes()
        })
        .sum();

    Ok(total_duration)
}

/// Calculates the current fasting streak for a specific user
pub fn calculate_current_streak(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<u32, FastingAppError> {
    // Retrieve fasting history ordered by start time (latest first)
    let events: Vec<FastingEvent> = fasting_events
        .filter(schema_user_id.eq(user_id)) // Using `schema_user_id` for the column
        .order(start_time.desc())
        .load(conn)
        .map_err(FastingAppError::DatabaseError)?;

    let mut streak = 0;
    let mut current_date = Utc::now().naive_utc().date();

    for event in events {
        let event_date = event.start_time.date();

        // Check if event_date matches current_date or the day before
        if event_date == current_date || Some(event_date) == current_date.pred_opt() {
            streak += 1;
            current_date = event_date;
        } else {
            break;
        }
    }

    Ok(streak)
}
