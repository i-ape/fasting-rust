use crate::errors::FastingAppError;
use crate::models::User;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::SqliteConnection;

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