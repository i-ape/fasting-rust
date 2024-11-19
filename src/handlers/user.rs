use crate::errors::FastingAppError;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::{hashed_password, username, id, users}; 
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::SqliteConnection;

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
/// Registers a new user
pub fn register_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<String, FastingAppError> {
    create_user(conn, username_input, password_input)
        .map(|_| "User created successfully".to_string())
        .map_err(|e| match e {
            FastingAppError::DatabaseError(diesel_error) => {
                FastingAppError::DatabaseError(diesel_error)
            }
            _ => e,
        })
}
/// Finds a user by username in the database
pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    use crate::schema::users::dsl::*;
    users
        .filter(username.eq(username_input))
        .first::<User>(conn) // Ensure the `User` struct matches the query
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}


/// Logs in a user by verifying credentials
pub fn login_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<User, FastingAppError> {
    let user = find_user_by_username(conn, username_input)?;
    if verify(password_input, &user.hashed_password).map_err(FastingAppError::PasswordHashError)? {
        Ok(user)
    } else {
        Err(FastingAppError::InvalidCredentials)
    }
}

/// Updates user profile details
pub fn update_user_profile(
    conn: &mut SqliteConnection,
    user_id: i32,
    new_username: Option<&str>,
    new_password: Option<&str>,
) -> Result<usize, FastingAppError> {
    // Return an error if no updates are provided
    if new_username.is_none() && new_password.is_none() {
        return Err(FastingAppError::InvalidRequest("No updates provided".to_string()));
    }

    let query = diesel::update(users.filter(id.eq(user_id)));

    if let Some(username_value) = new_username {
        if let Some(password_value) = new_password {
            // Update both username and password
            let hashed_password_value = hash(password_value, DEFAULT_COST)
                .map_err(FastingAppError::PasswordHashError)?;
            query
                .set((username.eq(username_value), hashed_password.eq(hashed_password_value)))
                .execute(conn)
                .map_err(FastingAppError::DatabaseError)
        } else {
            // Update only username
            query
                .set(username.eq(username_value))
                .execute(conn)
                .map_err(FastingAppError::DatabaseError)
        }
    } else if let Some(password_value) = new_password {
        // Update only password
        let hashed_password_value = hash(password_value, DEFAULT_COST)
            .map_err(FastingAppError::PasswordHashError)?;
        query
            .set(hashed_password.eq(hashed_password_value))
            .execute(conn)
            .map_err(FastingAppError::DatabaseError)
    } else {
        Err(FastingAppError::InvalidRequest("No updates provided".to_string()))
    }
}
