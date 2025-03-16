use crate::errors::FastingAppError;
use crate::models::NewUser;
use crate::schema::users::dsl::users;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// ✅ Creates a new user
pub fn create_user(
    conn: &mut SqliteConnection, 
    username_input: &str, 
    password_input: &str
) -> Result<(), FastingAppError> {
    let hashed_password = hash(password_input, DEFAULT_COST)
        .map_err(FastingAppError::PasswordHashError)?;

    let new_user = NewUser {
        username: username_input.to_string(),
        hashed_password,
        device_id: None,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)?;

    Ok(())
}
