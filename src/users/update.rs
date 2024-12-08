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
    if new_username.is_none() && new_password.is_none() && new_device_id.is_none() {
        return Err(FastingAppError::InvalidRequest(
            "No updates provided".to_string(),
        ));
    }

    let query = diesel::update(users.filter(id.eq(user_id)));

    // Build the update tuple dynamically
    let updates = (
        new_username.map(|username_value| username.eq(username_value)),
        new_password
            .map(|password_value| {
                let hashed_password_value = hash(password_value, DEFAULT_COST)
                    .map_err(FastingAppError::PasswordHashError)?;
                Ok::<_, FastingAppError>(hashed_password.eq(hashed_password_value))
            })
            .transpose()?, // Handle potential hashing errors
        new_device_id.map(|device_id_value| device_id.eq(device_id_value)),
    );

    // Apply the updates
    query
        .set(updates)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}
