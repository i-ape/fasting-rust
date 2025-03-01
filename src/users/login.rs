use crate::errors::FastingAppError;
use crate::models::User;
use crate::schema::users::dsl::*;
use bcrypt::verify;
use diesel::prelude::*;

/// ✅ Logs in a user by verifying their username and password.
///
/// - Calls `find_user_by_username` to get user details.
/// - Uses bcrypt to verify the password.
/// - Returns `User` if login is successful, otherwise returns `InvalidCredentials`
pub fn login_user(
    conn: &mut SqliteConnection,
    username_input: &str,
    password_input: &str,
) -> Result<User, FastingAppError> {
    let user = users
        .filter(username.eq(username_input))
        .first::<User>(conn)
        .map_err(FastingAppError::DatabaseError)?;

    if verify(password_input, &user.hashed_password).map_err(FastingAppError::PasswordHashError)? {
        Ok(user)
    } else {
        Err(FastingAppError::InvalidCredentials(format!(
            "Invalid login for username: {}",
            username_input
        )))
    }
}

/// 🔒 Finds a user by their **device ID**.
///
/// - had the fn of user login or device, now is a experiment.
/// - Returns `User` if the device ID exists, otherwise returns `InvalidCredentials`
pub fn login(
    conn: &mut SqliteConnection,
    username_input: Option<&str>,
    password_input: Option<&str>,
    device_id_input: Option<&str>,
) -> Result<User, FastingAppError> {
    if let Some(device_id_value) = device_id_input {
        return users
            .filter(device_id.eq(device_id_value))
            .first::<User>(conn)
            .optional()
            .map_err(FastingAppError::DatabaseError)?
            .ok_or_else(|| {
                FastingAppError::InvalidCredentials(format!(
                    "Device ID '{}' not found",
                    device_id_value
                ))
            });
    }

    if let (Some(input_username), Some(input_password)) = (username_input, password_input) {
        return login_user(conn, input_username, input_password);
    }

    Err(FastingAppError::InvalidRequest(
        "Must provide either a device ID or a username/password.".to_string(),
    ))
}

/// ✅ Associates a **device ID** with a user account.
///
/// - **PUBLIC** (`pub`): Called externally when linking devices.
/// - Ensures the device ID is not empty.
/// - Updates the database with the new device ID.
pub fn associate_device_id(
    conn: &mut SqliteConnection,
    user_id_input: i32,
    device_id_input: &str,
) -> Result<(), FastingAppError> {
    if device_id_input.is_empty() {
        return Err(FastingAppError::InvalidRequest(
            "Device ID cannot be empty.".to_string(),
        ));
    }

    diesel::update(users.filter(id.eq(user_id_input)))
        .set(device_id.eq(device_id_input))
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)?;

    Ok(())
}
