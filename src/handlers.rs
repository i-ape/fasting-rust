use crate::errors::FastingAppError;
use crate::models::{FastingEvent, NewFastingEvent, NewUser, User};
use crate::schema::{fasting_events::dsl::*, users::dsl::*};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;
pub mod user;
pub mod fasting;
pub mod analytics;

pub use user::*;
pub use fasting::*;
pub use analytics::*;


/// Helper function to map database errors
fn handle_db_error<T>(result: QueryResult<T>) -> Result<T, FastingAppError> {
    result.map_err(FastingAppError::DatabaseError)
}

/// Handler to register a new user
pub fn register_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<String, FastingAppError> {
    create_user(conn, username_input, password_input)
        .map(|_| "User created successfully".to_string())
        .map_err(|e| match e {
            FastingAppError::DatabaseError(diesel_error) => {
                FastingAppError::DatabaseError(diesel_error)
            }
            _ => e,
        })
}

/// Find user by username
pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    users
        .filter(username.eq(username_input))
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}

/// Create a new user with a hashed password
pub fn create_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<usize, FastingAppError> {
    let hashedp = hash(password_input, DEFAULT_COST).map_err(FastingAppError::PasswordHashError)?;
    // Create a new user struct with a different variable name
    let new_user = NewUser {
        username: username_input.to_string(),
        hashed_password: hashedp,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Log in the user by verifying username and password
pub fn login_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<User, FastingAppError> {
    let user = find_user_by_username(conn, username_input)?;
    if verify(password_input, &user.hashed_password).map_err(FastingAppError::PasswordHashError)? {
        Ok(user)
    } else {
        Err(FastingAppError::InvalidCredentials)
    }
}

/// Start a fasting event for a user
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id_input: i32,
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
}

/// Stop a fasting event for a user
pub fn stop_fasting(
    conn: &mut SqliteConnection,
    user_id_input: i32,
    end_time_input: NaiveDateTime,
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

/// Find an active fasting event for a specific user
fn find_active_fasting_event(
    conn: &mut SqliteConnection,
    user_id_input: i32,
) -> Result<Option<FastingEvent>, FastingAppError> {
    fasting_events
        .filter(user_id.eq(user_id_input))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)
}

/// Update user profile information
pub fn update_user_profile(
    conn: &mut SqliteConnection,
    user_id_input: i32,
    new_username: Option<&str>,
    new_password: Option<&str>,
) -> Result<usize, FastingAppError> {
    let mut query = diesel::update(users).into_boxed();

    if let Some(username) = new_username {
        query = query.set(username.eq(username.to_string()));
    }

    if let Some(password) = new_password {
        
    let hashedp = hash(password_input, DEFAULT_COST).map_err(FastingAppError::PasswordHashError)?;
    // Create a new user struct with a different variable name
    let new_user = NewUser {
        username: username_input.to_string(),
        hashed_password: hashedp,
    };
    }

    query.execute(conn).map_err(FastingAppError::DatabaseError)
}

/// Retrieve fasting history for a specific user
pub fn get_fasting_history(
    conn: &mut SqliteConnection,
    user_id_input: i32,
) -> Result<Vec<FastingEvent>, FastingAppError> {
    fasting_events
        .filter(user_id.eq(user_id_input))
        .order(start_time.desc())
        .load::<FastingEvent>(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Get the current fasting status for a user
pub fn get_current_fasting_status(
    conn: &mut SqliteConnection,
    user_id_input: i32,
) -> Result<Option<(NaiveDateTime, i64)>, FastingAppError> {
    if let Some(event) = find_active_fasting_event(conn, user_id_input)? {
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

/// Calculate the average fasting duration for a user
pub fn calculate_average_fasting_duration(
    conn: &mut SqliteConnection,
    user_id_input: i32,
) -> Result<Option<i64>, FastingAppError> {
    let events: Vec<FastingEvent> = fasting_events
        .filter(user_id.eq(user_id_input))
        .filter(stop_time.is_not_null())
        .load(conn)
        .map_err(FastingAppError::DatabaseError)?;

    let total_duration: i64 = events
        .iter()
        .map(|event| {
            let duration = event
                .stop_time
                .unwrap()
                .signed_duration_since(event.start_time)
                .num_minutes();
            duration
        })
        .sum();

    if events.is_empty() {
        Ok(None)
    } else {
        Ok(Some(total_duration / events.len() as i64))
    }
}
