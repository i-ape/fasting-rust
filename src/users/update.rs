use crate::errors::FastingAppError;
use crate::schema::users::dsl::{device_id, hashed_password, username, id, users};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// Updates user profile details.
pub fn update_user_profile(
    conn: &mut SqliteConnection,
    user_id: i32,
    new_username: Option<&str>,
    new_password: Option<&str>,
    new_device_id: Option<&str>,
) -> Result<usize, FastingAppError> {
    // Ensure at least one update is provided
    if new_username.is_none() && new_password.is_none() && new_device_id.is_none() {
        return Err(FastingAppError::InvalidRequest(
            "No updates provided.".to_string(),
        ));
    }

    let mut updates = Vec::new();

    // Add username update if provided
    if let Some(username_value) = new_username {
        updates.push(username.eq(username_value));
    }

    // Add password update if provided
    if let Some(password_value) = new_password {
        let hashed_password_value = hash(password_value, DEFAULT_COST)
            .map_err(FastingAppError::PasswordHashError)?;
        updates.push(hashed_password.eq(hashed_password_value));
    }

    // Add device ID update if provided
    if let Some(device_id_value) = new_device_id {
        updates.push(device_id.eq(device_id_value));
    }

    // Execute the update query with the constructed updates
    diesel::update(users.filter(id.eq(user_id)))
        .set(updates)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}
