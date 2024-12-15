use crate::errors::FastingAppError;
use crate::schema::users::dsl::{device_id, hashed_password, id, username, users};
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

    // Dynamically build the update tuple
    let mut query = diesel::update(users.filter(id.eq(user_id)));

    if let Some(username_value) = new_username {
        query = query.set(username.eq(username_value));
    }

    if let Some(password_value) = new_password {
        let hashed_password_value = hash(password_value, DEFAULT_COST)
            .map_err(FastingAppError::PasswordHashError)?;
        query = query.set(hashed_password.eq(hashed_password_value));
    }

    if let Some(device_id_value) = new_device_id {
        query = query.set(device_id.eq(device_id_value));
    }

    // Execute the query
    query
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}
