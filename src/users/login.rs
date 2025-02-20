use crate::errors::FastingAppError;
use crate::models::User;
use bcrypt::verify;
use diesel::prelude::*;
use crate::schema::users::dsl::*;
use super::find::find_user_by_username;

/// Logs in a user by verifying their username and password.
pub fn login_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<User, FastingAppError> {
    let user = find_user_by_username(conn, username_input)?;

    // Verify the password
    if verify(password_input, &user.hashed_password).map_err(FastingAppError::PasswordHashError)? {
        Ok(user)
    } else {
        Err(FastingAppError::InvalidCredentials(format!("Invalid login for username: {}", username_input)))
    }
}

/// Finds a user by their device ID.
pub fn find_user_by_device_id(
    conn: &mut SqliteConnection,
    device_id_input: &str,
) -> Result<User, FastingAppError> {
    users
        .filter(device_id.eq(device_id_input))
        .select(User::as_select())
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or_else(|| FastingAppError::InvalidCredentials(format!("Device ID '{}' not found", device_id_input)))
}

/// Associates a device ID with a user account.
pub fn associate_device_id(
    conn: &mut SqliteConnection,
    user_id_input: i32,
    device_id_input: &str,
) -> Result<usize, FastingAppError> {
    if device_id_input.is_empty() {
        return Err(FastingAppError::InvalidRequest("Device ID cannot be empty.".to_string()));
    }

    diesel::update(users.filter(id.eq(user_id_input)))
        .set(device_id.eq(device_id_input))
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Logs in using either username/password or device ID.
pub fn login_user_or_device(
    conn: &mut SqliteConnection,
    username_input: Option<&str>,
    password_input: Option<&str>,
    device_id_input: Option<&str>,
) -> Result<User, FastingAppError> {
    match device_id_input {
        Some(device_id_value) => find_user_by_device_id(conn, device_id_value),
        None => {
            if let (Some(input_username), Some(input_password)) = (username_input, password_input) {
                login_user(conn, input_username, input_password)
            } else {
                Err(FastingAppError::InvalidRequest(
                    "Must provide either a device ID or a username/password.".to_string(),
                ))
            }
        }
    }
}
