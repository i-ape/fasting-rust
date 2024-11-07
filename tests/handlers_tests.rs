use chrono::NaiveDateTime;
use diesel::connection::Connection;
use diesel::sqlite::SqliteConnection;
use fasting_rust::errors::FastingAppError;
use fasting_rust::handlers::{
    create_user, find_user_by_username, login_user, start_fasting, stop_fasting,
};
use fasting_rust::models::{NewUser, User};
use fasting_rust::schema::users::dsl::*;

fn establish_test_connection() -> SqliteConnection {
    let conn = SqliteConnection::establish(":memory:").unwrap();
    // Run schema migrations here if using Diesel migrations
    conn
}

#[test]
fn test_create_user() {
    let mut conn = establish_test_connection();
    let username = &str;
    let password: &str = &str;

    let result = create_user(&mut conn, username, password);
    assert!(result.is_ok());

    let inserted_user: User = users
        .filter(username.eq(username))
        .first(&mut conn)
        .expect("User should be inserted");

    assert_eq!(inserted_user.username, username);
}

#[test]
fn test_login_user_success() {
    let mut conn = establish_test_connection();
    let username = &str;
    let password: &str = &str;

    create_user(&mut conn, username, password).unwrap();
    let user = login_user(&mut conn, username, password);
    assert!(user.is_ok());
}

// Additional tests for start_fasting, stop_fasting, etc.
