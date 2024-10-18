use diesel::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;
use crate::models::{User, NewUser, NewFastingEvent};
use crate::schema::users::dsl::*;
use crate::schema::fasting_events::dsl::*;

// Create a new user with hashed password
pub fn create_user(conn: &SqliteConnection, username_input: &str, password_input: &str) -> Result<usize, diesel::result::Error> {
    let hashed_password = hash(password_input, DEFAULT_COST).expect("Error hashing password");

    let new_user = NewUser {
        username: username_input.to_string(),
        hashed_password: hashed_password,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)
}

// Log in the user
pub fn login_user(conn: &SqliteConnection, username_input: &str, password_input: &str) -> Result<User, diesel::result::Error> {
    let user = users.filter(username.eq(username_input)).first::<User>(conn)?;

    if verify(password_input, &user.hashed_password).unwrap_or(false) {
        Ok(user)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

// Start a fasting session
pub fn start_fasting(conn: &SqliteConnection, user_id_input: i32, start_time: NaiveDateTime) -> Result<usize, diesel::result::Error> {
    let new_event = NewFastingEvent {
        user_id: user_id_input,
        start_time: start_time,
        stop_time: None,
    };

    diesel::insert_into(fasting_events).values(&new_event).execute(conn)
}

// Stop a fasting session
pub fn stop_fasting(conn: &SqliteConnection, event_id: i32, stop_time_input: NaiveDateTime) -> Result<usize, diesel::result::Error> {
    diesel::update(fasting_events.find(event_id))
        .set(stop_time.eq(Some(stop_time_input)))
        .execute(conn)
}
