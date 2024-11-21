use crate::errors::FastingAppError;
use crate::models::User;
use bcrypt::verify;
use diesel::SqliteConnection;

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
