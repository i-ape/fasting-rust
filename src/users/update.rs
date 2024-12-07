use crate::errors::FastingAppError;
use crate::schema::users::dsl::{hashed_password, username, id, users};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Updates user profile details.
pub fn update_user_profile(
    conn: &mut SqliteConnection,
    user_id: i32,
    new_username: Option<&str>,
    new_password: Option<&str>,
    new_device_id: Option<&str>,
) -> Result<usize, FastingAppError> {
    if new_username.is_none() && new_password.is_none() {
        return Err(FastingAppError::InvalidRequest("No updates provided".to_string()));
    }

    let query = diesel::update(users.filter(id.eq(user_id)));

    if let Some(username_value) = new_username {
        if let Some(password_value) = new_password {
            let hashed_password_value = hash(password_value, DEFAULT_COST)
                .map_err(FastingAppError::PasswordHashError)?;
            query
                .set((username.eq(username_value), hashed_password.eq(hashed_password_value)))
                .execute(conn)
                .map_err(FastingAppError::DatabaseError)
        } else {
            query
                .set(username.eq(username_value))
                .execute(conn)
                .map_err(FastingAppError::DatabaseError)
        }
    } else if let Some(password_value) = new_password {
        let hashed_password_value = hash(password_value, DEFAULT_COST)
            .map_err(FastingAppError::PasswordHashError)?;
        query
            .set(hashed_password.eq(hashed_password_value))
            .execute(conn)
            .map_err(FastingAppError::DatabaseError)
    } else {
        Err(FastingAppError::InvalidRequest("No updates provided".to_string()))
    }
}
