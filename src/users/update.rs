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

    // Prepare updates as a tuple
    let updates = (
        new_username.map(|val| username.eq(val)),
        new_password.map(|val| {
            let hashed_password_value = hash(val, DEFAULT_COST)
                .map_err(FastingAppError::PasswordHashError)?;
            Ok::<_, FastingAppError>(hashed_password.eq(hashed_password_value))
        })
        .transpose()?, // Handle password hashing errors
        new_device_id.map(|val| device_id.eq(val)),
    );

    // Execute the query with the tuple
    diesel::update(users.filter(id.eq(user_id)))
        .set(updates)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)
}
