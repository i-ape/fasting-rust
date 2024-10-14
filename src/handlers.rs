use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::models::{FastingEvent, NewFastingEvent, NewUser, User};
use crate::schema::fasting_events::dsl::*;
use crate::schema::users::dsl::*;
use chrono::{NaiveDateTime, Utc};

/// Create a new user in the database, with hashed password
pub fn create_user(
    conn: &SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<usize, diesel::result::Error> {
    // Hash the password before storing it
    let hashed_password = hash(password, DEFAULT_COST).expect("Error hashing password");

    let new_user = NewUser {
        username: str,
        hashed_password: str,
    };

    diesel::insert_into(users).values(&new_user).execute(&conn)
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
    if verify(password_input, &user.hashed_password).unwrap_or(false) {
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
) -> Result<usize, diesel::result::Error> {
    let new_event = NewFastingEvent {
        user_id: user_id_input,
        start_time,
        end_time: None, // fast is ongoing
    };

    diesel::insert_into(fasting_events)
        .values(&new_event)
        .execute(conn)
}

/// Stop fasting event for a user (marks the end of a fast)
pub fn stop_fasting(
    conn: &SqliteConnection,
    event_id: i32,
    stop_time: NaiveDateTime,
) -> Result<usize, diesel::result::Error> {
    // Update the fasting event, setting the end_time to stop_time
    diesel::update(fasting_events.find(event_id))
        .set(end_time.eq(Some(stop_time)))
        .execute(conn)
}
