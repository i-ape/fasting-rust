use crate::errors::FastingAppError;
use crate::models::{FastingEvent, NewFastingEvent, NewUser, User};
use crate::schema::fasting_events::dsl::*;
use crate::schema::users::dsl::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::SqliteConnection;

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

/// Log in the user by verifying the username and password
pub fn login_user(
    conn: &mut SqliteConnection, // Mutable reference to conn
    username_input: &str,
    password_input: &str,
) -> Result<User, FastingAppError> {
    let user = users
        .filter(username.eq(username_input))
        .first::<User>(conn) // No type mismatch here if conn is mutable
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)?;

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
    event_start_time: NaiveDateTime, // Renamed to avoid conflict
) -> Result<usize, FastingAppError> {
    let active_event = fasting_events
        .filter(user_id.eq(user_id_input))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?;

    if active_event.is_some() {
        return Err(FastingAppError::ExistingSessionError);
    }

    let new_event = NewFastingEvent {
        user_id: user_id_input,
        start_time: event_start_time, // Use the new variable name here
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
    diesel::update(fasting_events.filter(user_id.eq(user_id_input).and(stop_time.is_null())))
        .set(stop_time.eq(Some(end_time_input)))
        .execute(conn) // `conn` is mutable here
        .map_err(FastingAppError::DatabaseError)
}
