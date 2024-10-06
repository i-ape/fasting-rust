use diesel::prelude::*;
use crate::schema::users;
use crate::db::establish_connection;
use crate::models::{NewUser, User};

// Function implementations


pub fn create_user(_connection: &SqliteConnection, username: &str, password: &str) {
    println!("Creating user: {} with password: {}", username, password);
    let _conn = establish_connection();

    diesel::insert_into(users)
        .values(&new_user)
        .execute(_conn)
        .expect("Error creating user");
}

pub fn start_fasting(user_id: i32) {
    println!("Starting fasting session for user ID: {}", user_id);
}

pub fn stop_fasting(session_id: i32) {
    println!("Stopping fasting session for session ID: {}", session_id);
}
