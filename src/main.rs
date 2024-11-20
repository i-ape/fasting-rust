extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use chrono::Utc;
use dotenv::dotenv;
use std::io::{self, Write};

mod handlers;
use handlers::{create_user, login_user, start_fasting, stop_fasting};

mod db;
mod errors;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::errors::FastingAppError;

fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn handle_error(error: FastingAppError) {
    match error {
        FastingAppError::InvalidRequest(msg) => {
            println!("Invalid request: {}", msg);
        }
        FastingAppError::DatabaseError(e) => {
            println!("Database error: {}", e);
        }
        FastingAppError::PasswordHashError(e) => {
            println!("Password hash error: {}", e);
        }
        FastingAppError::ExistingSessionError => {
            println!("An existing fasting session is already active.");
        }
        FastingAppError::InvalidCredentials => {
            println!("Invalid username or password.");
        }
    }
}

fn manage_fasting_session(conn: &mut diesel::SqliteConnection, user_id: i32) {
    // Start a fasting session
    match start_fasting(conn, user_id, Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session started."),
        Err(e) => handle_error(e),
    }

    // Stop the fasting session
    match stop_fasting(conn, user_id, Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session stopped."),
        Err(e) => handle_error(e),
    }
}

fn main() {
    dotenv().ok();

    // Establish the database connection
    let mut conn = establish_connection();

    // Get input from the user
    let username = prompt_input("Enter username: ");
    let password = prompt_input("Enter password: ");

    // Create a new user
    if let Err(e) = create_user(&mut conn, &username, &password) {
        handle_error(e);
    } else {
        println!("User created successfully.");
    }

    // Log in the user
    match login_user(&mut conn, &username, &password) {
        Ok(user) => {
            if let Some(id) = user.id {
                println!("Login successful. User ID: {}", id);
                manage_fasting_session(&mut conn, id);
            } else {
                println!("Login successful, but User ID is not available.");
            }
        }
        Err(e) => handle_error(e),
    }

    // Example error handling for another function
    if let Err(error) = some_function_that_might_fail() {
        handle_error(error);
    }
}
