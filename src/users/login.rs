use crate::errors::FastingAppError;
use crate::models::User;
use bcrypt::verify;
use diesel::SqliteConnection;
use crate::schema::users::dsl::{device_id, id, users};
use super::find::find_user_by_username;

/// Logs in a user by verifying their credentials.
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
/// Finds a user by their device ID
pub fn find_user_by_device_id(
    conn: &mut SqliteConnection,
    device_id_input: &str,
) -> Result<User, FastingAppError> {
    users
        .filter(device_id.eq(device_id_input))
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}
/// Associates a device ID with a user
pub fn associate_device_id(
    conn: &mut SqliteConnection,
    user_id: i32,
    device_id_input: &str,
) -> Result<usize, FastingAppError> {
    diesel::update(users.filter(id.eq(user_id)))
        .set(device_id.eq(device_id_input))
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}
/// Login using either username/password or device ID
pub fn login_user_or_device(
    conn: &mut SqliteConnection,
    username_input: Option<&str>,
    password_input: Option<&str>,
    device_id_input: Option<&str>,
) -> Result<User, FastingAppError> {
    if let Some(device_id) = device_id_input {
        return find_user_by_device_id(conn, device_id);
    }

    if let (Some(username), Some(password)) = (username_input, password_input) {
        return super::login::login_user(conn, username, password);
    }

    Err(FastingAppError::InvalidRequest(
        "Must provide either device ID or username/password.".to_string(),
    ))
}