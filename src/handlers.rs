use bcrypt::{hash, verify};
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::models::{NewUser, User};
use crate::schema::fasting_sessions;
use crate::schema::users;
use crate::schema::users::dsl::*; // Using `dsl` to simplify references to columns
//use chrono::NaiveDateTime;

/// Create a new user and insert it into the database
pub fn create_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<usize, diesel::result::Error> {
    let hashed_password = hash(password, 4).expect("Error hashing password");

    let new_user = NewUser {
        username: username.to_string(),
        hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}

/// Log in a user by verifying their username and password
pub fn login_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<bool, diesel::result::Error> {
    let user: User = users.filter(username.eq(input_username)).first(conn)?;

    let is_password_valid = verify(input_password, &user.hashed_password).unwrap();
    Ok(is_password_valid)
}

/// Start a fasting session for a user
pub fn start_fasting(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<usize, diesel::result::Error> {
    let new_session = crate::models::NewFastingSession {
        user_id,
        start_time: chrono::Utc::now().naive_utc(),
        end_time: None,
    };

    diesel::insert_into(fasting_sessions::table)
        .values(&new_session)
        .execute(conn)
}

/// Stop a fasting session by setting the `end_time`
pub fn stop_fasting(
    conn: &SqliteConnection,
    session_id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(
        fasting_sessions::dsl::fasting_sessions.filter(fasting_sessions::dsl::id.eq(session_id)),
    )
    .set(fasting_sessions::dsl::end_time.eq(Some(chrono::Utc::now().naive_utc())))
    .execute(conn)
}
