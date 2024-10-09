use crate::db::establish_connection;
use crate::models::{FastingSession, NewFastingSession, NewUser, User};
use crate::schema::{fasting_sessions, users};
use bcrypt::{hash, verify};
use chrono::{NaiveDateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;

// Register a new user in the database
pub fn create_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<User, diesel::result::Error> {
    let hashed_password = hash(password, 4).unwrap(); // Hash password with bcrypt
    let new_user = NewUser {
        username: username.to_string(),
        password: hashed_password,
    };

    insert_into(users::table).values(&new_user).execute(conn)?;

    users::table.order(users::id.desc()).first(conn)
}

// Login user by verifying the password
pub fn login_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<User, &'static str> {
    let user: User = users::table
        .filter(users::username.eq(username))
        .first(conn)
        .map_err(|_| "User not found")?;
    if verify(password, &user.password).unwrap() {
        Ok(user)
    } else {
        Err("Invalid password")
    }
}

// Start a fasting session for a user
pub fn start_fasting(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<FastingSession, diesel::result::Error> {
    // Check if a fasting session is already active for this user
    if let Some(_) = get_active_session(conn, user_id)? {
        return Err(diesel::result::Error::NotFound);
    }

    let new_session = NewFastingSession {
        user_id,
        start_time: Utc::now().naive_utc(),
        end_time: None,
    };
    insert_into(fasting_sessions::table)
        .values(&new_session)
        .execute(conn)?;

    fasting_sessions::table
        .order(fasting_sessions::id.desc())
        .first(conn)
}

// Stop an active fasting session by setting the end time
pub fn stop_fasting(
    conn: &SqliteConnection,
    session_id: i32,
) -> Result<FastingSession, diesel::result::Error> {
    diesel::update(fasting_sessions::table.filter(fasting_sessions::id.eq(session_id)))
        .set(fasting_sessions::end_time.eq(Some(Utc::now().naive_utc())))
        .execute(conn)?;

    fasting_sessions::table
        .filter(fasting_sessions::id.eq(session_id))
        .first(conn)
}

// Get all fasting sessions for a user
pub fn get_session_history(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<Vec<FastingSession>, diesel::result::Error> {
    fasting_sessions::table
        .filter(fasting_sessions::user_id.eq(user_id))
        .load::<FastingSession>(conn)
}

// Get the active fasting session for a user, if any
pub fn get_active_session(
    conn: &SqliteConnection,
    user_id: i32,
) -> Result<Option<FastingSession>, diesel::result::Error> {
    let active_session = fasting_sessions::table
        .filter(fasting_sessions::user_id.eq(user_id))
        .filter(fasting_sessions::end_time.is_null())
        .first::<FastingSession>(conn)
        .optional()?;

    Ok(active_session)
}

// Helper function to calculate the duration of a fasting session
pub fn calculate_session_duration(start_time: NaiveDateTime, end_time: NaiveDateTime) -> String {
    let duration = end_time.signed_duration_since(start_time);
    format!(
        "{} hours, {} minutes",
        duration.num_hours(),
        duration.num_minutes() % 60
    )
}
