use crate::errors::FastingAppError;
use crate::models::{FastingEvent, NewFastingEvent, NewUser, User};
use crate::schema::{fasting_events::dsl::*, users::dsl::*};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;

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
        .map_err(|e| {
            match e {
                FastingAppError::DatabaseError(_) => FastingAppError::DatabaseError("Failed to create user.".to_string()),
                _ => e,
            }
        })
}
pub fn find_user_by_username_alt(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    users
        .filter(username.eq(username_input))
        //.select(User::as_select())
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}
/// Helper function to find an active fasting event for a specific user
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

/// Create a new user in the database with a hashed password
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

/// Find a user by username
pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    users::table
        .filter(users::username.eq(username_input)) // Use `users::username`
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}

/// Log in the user by verifying the username and password
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

/// Start fasting event for a user
pub fn start_fasting(
    conn: &mut SqliteConnection,
    user_id_input: i32,
    event_start_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    // Check if there's an active fasting event
    if find_active_fasting_event(conn, user_id_input)?.is_some() {
        return Err(FastingAppError::ExistingSessionError);
    }

    // Create and insert a new fasting event
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

/// Stop fasting event for a user (marks the end of a fast)
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