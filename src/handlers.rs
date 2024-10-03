use crate::models::User;
use crate::schema::users::dsl::*;
use diesel::insert_into;

pub fn create_user(conn: &SqliteConnection, username: &str, hashed_password: &str) -> User {
    let new_user = User {
        id: 0,
        username: username.to_string(),
        hashed_password: hashed_password.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error creating new user");

    new_user
}
