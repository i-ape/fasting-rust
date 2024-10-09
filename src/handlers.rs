use diesel::prelude::*;
use diesel::insert_into;
use diesel::result::Error;
use crate::models::{User, NewUser};
use crate::schema::users;
use diesel::sqlite::SqliteConnection;
use bcrypt::{hash, verify};

// Registers a new user with a given `username` and `password`.
pub fn create_user(
    conn: &mut SqliteConnection, 
    username: &str, 
    password: &str
) -> Result<User, Error> {
    // Hash the password using bcrypt
    let hashed_password = hash(password, 4).expect("Failed to hash password");

    // Create a new `NewUser` instance
    let new_user = NewUser {
        username: username.to_string(),
        password: hashed_password,
    };

    // Insert the new user into the `users` table
    insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    // Retrieve and return the newly created user
    users::table
        .order(users::id.desc())
        .first(conn)
}

// Authenticates a user by `username` and `password`.
pub fn login_user(
    conn: &mut SqliteConnection, 
    username: &str, 
    password: &str
) -> Result<bool, Error> {
    // Query the user by `username`
    let user = users::table
        .filter(users::username.eq(username))
        .first::<User>(conn)?;

    // Verify the provided password against the stored hashed password
    let is_valid = verify(password, &user.password).expect("Failed to verify password");

    Ok(is_valid)
}
