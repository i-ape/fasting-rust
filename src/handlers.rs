use crate::errors::FastingAppError;
use crate::models::{FastingEvent, NewFastingEvent, NewUser, User};
use crate::schema::fasting_events::dsl::*;
use crate::schema::users::dsl::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Create a new user in the database with a hashed password
pub fn create_user(
    conn: &SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<usize, FastingAppError> {
    let hashed_password =
        hash(password_input, DEFAULT_COST).map_err(FastingAppError::PasswordHashError)?;

    let new_user = NewUser {
        username: username_input.to_string(),
        hashed_password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Log in the user by verifying the username and password
pub fn login_user(
    conn: &SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<User, diesel::result::Error> {
    // Query for the user by username
    let user = users
        .filter(username.eq(username_input))
        .first::<User>(conn)?;

    // Verify the password
    if verify(password_input, &user.hashed_password)
        .map_err(|_| diesel::result::Error::RollbackTransaction)?
    {
        Ok(user)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

/// Start fasting event for a user
pub fn start_fasting(
    conn: &SqliteConnection,
    user_id_input: i32,
    start_time: NaiveDateTime,
) -> Result<usize, FastingAppError> {
    let active_event = fasting_events
        .filter(user_id.eq(user_id_input))
        .filter(stop_time.is_null())
        .first::<FastingEvent>(conn)
        .optional()?;

    if active_event.is_some() {
        return Err(FastingAppError::ExistingSessionError);
    }

    let new_event = NewFastingEvent {
        user_id: user_id_input,
        start_time,
        stop_time: None,
    };

    diesel::insert_into(fasting_events)
        .values(&new_event)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Stop fasting event for a user (marks the end of a fast)
pub fn stop_fasting(
    conn: &SqliteConnection,
    user_id_input: i32,
    stop_time: NaiveDateTime,
) -> Result<usize, diesel::result::Error> {
    // Update the fasting event, setting the stop_time
    diesel::update(fasting_events.filter(user_id.eq(user_id_input).and(stop_time.is_null())))
        .set(stop_time.eq(Some(stop_time)))
        .execute(conn)
}
