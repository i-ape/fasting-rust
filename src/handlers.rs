use bcrypt::{hash, verify};
use diesel::prelude::*;
use diesel::RunQueryDsl; // Import bcrypt for password hashing

use crate::models::{NewUser, User};
use crate::schema::fasting_sessions;
use crate::schema::users; // Import `users` table schema
use crate::schema::users::dsl::*; // Simplify referencing `users` columns
use chrono::NaiveDateTime;

/// Create a new user and insert it into the database
pub fn create_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<usize, diesel::result::Error> {
    // Hash the password before inserting
    let hashed_password = hash(password, 4).expect("Error hashing password");

    let new_user = NewUser {
        username: username.to_string(),
        hashed_password,
    };

    // Insert the new user into the `users` table
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn) // Use `conn` instead of `connection`
}

/// Log in a user by verifying their username and password
pub fn login_user(
    conn: &SqliteConnection,
    input_username: &str,
    input_password: &str,
) -> Result<bool, diesel::result::Error> {
    // Find the user by username
    let user: User = users.filter(username.eq(input_username)).first(conn)?;

    // Verify the password
    let is_password_valid = verify(input_password, &user.hashed_password).unwrap();
    Ok(is_password_valid)
}

/// Start a fasting session for a user
pub fn start_fasting(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<usize, diesel::result::Error> {
    use crate::models::NewFastingSession;

    let new_session = NewFastingSession {
        user_id,
        start_time: chrono::Utc::now().naive_utc(),
        end_time: None,
    };

    // Insert the fasting session into the `fasting_sessions` table
    diesel::insert_into(fasting_sessions::table)
        .values(&new_session)
        .execute(conn)
}

/// Stop a fasting session by setting the `end_time`
pub fn stop_fasting(
    conn: &SqliteConnection,
    session_id: i32,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::fasting_sessions::dsl::*;

    diesel::update(fasting_sessions.filter(id.eq(session_id)))
        .set(end_time.eq(Some(chrono::Utc::now().naive_utc())))
        .execute(conn)
}
