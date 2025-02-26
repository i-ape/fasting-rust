use crate::errors::FastingAppError;
use crate::models::User;
use diesel::prelude::*;
use crate::schema::users::dsl::*;

/// Finds a user by their username.  
/// 🔒 **PRIVATE**: Used only inside `login.rs`
pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    username_input: &str,
) -> Result<User, FastingAppError> {
    users
        .filter(username.eq(username_input))
        .select(User::as_select())
        .first::<User>(conn)
        .map_err(FastingAppError::DatabaseError)
}

/// Finds a user by their ID.
/// ✅ **PUBLIC**: If other modules (like analytics) need user lookup
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
