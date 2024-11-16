use crate::errors::FastingAppError;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::{username as schema_username, users};
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
    users
        .filter(username.eq(username_input))
        .first::<User>(conn)
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
    let mut query = diesel::update(users.filter(crate::schema::users::dsl::id.eq(user_id))).into_boxed();

    // Check if new_username is provided and apply it to the query
    if let Some(username) = new_username {
        query = query.set(crate::schema::users::dsl::username.eq(username.to_string()));
    }

    // Check if new_password is provided and apply it to the query
    if let Some(password) = new_password {
        let hashed_password = hash(password, DEFAULT_COST).map_err(FastingAppError::PasswordHashError)?;
        query = query.set(crate::schema::users::dsl::hashed_password.eq(hashed_password));
    }

    // Ensure at least one `SET` clause is applied
    if new_username.is_none() && new_password.is_none() {
        return Err(FastingAppError::InvalidRequest("No updates provided".into()));
    }

    // Execute the query
    query.execute(conn).map_err(FastingAppError::DatabaseError)
}
