use crate::errors::FastingAppError;
use crate::models::User;
use diesel::prelude::*;
use crate::schema::users::dsl::*;

/// ✅ Finds a user by their **ID**.
/// - **Public**: Used in multiple modules (analytics, fasting, etc.).
/// - Returns `FastingAppError::DatabaseError` if user is not found.
pub fn find_user_by_id(
    conn: &mut SqliteConnection,
    user_id_input: i32,
) -> Result<User, FastingAppError> {
    users
        .filter(id.eq(user_id_input))
        .select(User::as_select())
        .first::<User>(conn)
        .map_err(FastingAppError::DatabaseError)
}
