use crate::db::establish_connection;
use crate::models::{FastingSession, NewUser, User};
use crate::schema::fasting_sessions::dsl::*;
use crate::schema::users::dsl::*;
use bcrypt::{hash, verify};
use diesel::insert_into;
use diesel::prelude::*;

// Create a new user and insert into the database
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

    insert_into(users).values(&new_user).execute(conn)?;

    // Retrieve and return the newly created user
    users.order(id.desc()).first(conn)
}

// Login user by verifying the password
pub fn login_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<User, &'static str> {
    let user: User = users
        .filter(username.eq(username))
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
    let new_session = FastingSession {
        user_id,
        start_time: chrono::Utc::now().naive_utc(),
        end_time: None,
    };
    insert_into(fasting_sessions)
        .values(&new_session)
        .execute(conn)?;

    // Retrieve and return the newly created session
    fasting_sessions.order(id.desc()).first(conn)
}

// Stop an active fasting session by setting the end time
pub fn stop_fasting(
    conn: &SqliteConnection,
    session_id: i32,
) -> Result<FastingSession, diesel::result::Error> {
    use crate::schema::fasting_sessions::dsl::{end_time, id as session_id_column};

    diesel::update(fasting_sessions.filter(session_id_column.eq(session_id)))
        .set(end_time.eq(Some(chrono::Utc::now().naive_utc())))
        .execute(conn)?;

    // Return the updated session
    fasting_sessions
        .filter(session_id_column.eq(session_id))
        .first(conn)
}
