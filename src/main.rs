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

fn some_function_that_might_fail() -> Result<(), FastingAppError> {
    Err(FastingAppError::InvalidRequest("Example error".to_string()))
}

fn main() {
    dotenv().ok();

    // Establish the database connection
    let mut conn = establish_connection();

    // Get input from the user
    let username = prompt_input("Enter username: ");
    let password = prompt_input("Enter password: ");

    // Create a new user
    match create_user(&mut conn, &username, &password) {
        Ok(_) => println!("User created successfully."),
        Err(e) => println!("Error creating user: {:?}", e),
    }

    // Log in the user
    match login_user(&mut conn, &username, &password) {
        Ok(user) => {
            match user.id {
                Some(id) => println!("Login successful. User ID: {}", id),
                None => println!("Login successful. User ID is not available."),
            }

            // Start a fasting session
            match start_fasting(&mut conn, user.id.unwrap_or(-1), Utc::now().naive_utc()) {
                Ok(_) => println!("Fasting session started."),
                Err(e) => println!("Error starting fasting session: {:?}", e),
            }

            // Stop the fasting session
            match stop_fasting(&mut conn, user.id.unwrap_or(-1), Utc::now().naive_utc()) {
                Ok(_) => println!("Fasting session stopped."),
                Err(e) => println!("Error stopping fasting session: {:?}", e),
            }
        }
        Err(e) => println!("Error logging in: {:?}", e),
    }

    // Example error handling for another function
    if let Err(error) = some_function_that_might_fail() {
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
        }
    }
}
