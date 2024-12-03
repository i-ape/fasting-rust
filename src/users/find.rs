use crate::errors::FastingAppError;
use crate::models::User;
use crate::schema::users::dsl::{username, users};
use diesel::prelude::*;
use diesel::SqliteConnection;

/// Finds a user by their username in the database.
pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    users
        .filter(username.eq(username_input))
        .select(User::as_select())
        .first::<User>(conn)
        .optional()
        .map_err(FastingAppError::DatabaseError)?
        .ok_or(FastingAppError::InvalidCredentials)
}