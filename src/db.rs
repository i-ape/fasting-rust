// Imports from Diesel, dotenv, env, etc.
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*; // Add other diesel imports here
use dotenv::dotenv;
use std::env;
use crate::errors::FastingAppError;
use crate::models::{User, FastingEvent, NewUser, NewFastingEvent};
use crate::schema::{fasting_events::dsl::*, users::dsl::*};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDateTime;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// Establishes a direct connection to SQLite
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Pool-based connection for multithreaded applications
pub fn establish_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Function to find a user by username
pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    users.filter(username.eq(username_input))
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}
