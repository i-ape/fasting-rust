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

    // Collect updates into a tuple dynamically
    let mut update_query = diesel::update(users.filter(id.eq(user_id)));

    if let Some(username_value) = new_username {
        update_query = update_query.set(username.eq(username_value));
    }

    if let Some(password_value) = new_password {
        let hashed_password_value = hash(password_value, DEFAULT_COST)
            .map_err(FastingAppError::PasswordHashError)?;
        update_query = update_query.set(hashed_password.eq(hashed_password_value));
    }

    if let Some(device_id_value) = new_device_id {
        update_query = update_query.set(device_id.eq(device_id_value));
    }

    // Execute the query
    update_query
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}
