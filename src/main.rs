extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use chrono::Utc;
use dotenv::dotenv;
use std::io::{self, Write};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod users;

use crate::db::establish_connection;
use crate::errors::FastingAppError;
use fasting_rust::users::{
    create_user, find_user_by_username, login_user, register_user, update_user_profile,
};
use crate::handlers::{start_fasting, stop_fasting};

/// Prompts the user for input with the given message.
fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Handles application-level errors in a centralized way.
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
        FastingAppError::ConnectionError(err) => {
            println!("Failed to connect to the database: {}", err);
        }
    }
}

/// Manages a user's fasting session (start and stop).
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

/// Demonstrates the `ConnectionError` variant for testing.
fn demonstrate_connection_error() {
    let connection_error = errors::FastingAppError::ConnectionError;
    println!("Demonstration Error: {}", connection_error); // Output: Failed to connect to the database.
}

fn main() {
    dotenv().ok();

    // Call the demonstration function at the beginning of the main function
    demonstrate_connection_error();

    // Establish the database connection
    let mut conn = establish_connection();

    loop {
        // Prompt the user for an action
        let choice = prompt_input("\nChoose an action: register, login, update, or exit: ").to_lowercase();

        match choice.as_str() {
            "register" => {
                let username = prompt_input("Enter a new username: ");
                let password = prompt_input("Enter a new password: ");

                match register_user(&mut conn, &username, &password) {
                    Ok(_) => println!("User registered successfully."),
                    Err(e) => handle_error(e),
                }
            }
            "login" => {
                let username = prompt_input("Enter your username: ");
                let password = prompt_input("Enter your password: ");

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
            }
            "update" => {
                let user_id: i32 = prompt_input("Enter your User ID: ").parse().unwrap_or(-1);
                if user_id < 0 {
                    println!("Invalid User ID.");
                    continue;
                }

                let mut new_username = Some(prompt_input("Enter a new username (or press Enter to skip): "));
                let mut new_password = Some(prompt_input("Enter a new password (or press Enter to skip): "));

                if new_username.as_deref() == Some("") {
                    new_username = None;
                }
                if new_password.as_deref() == Some("") {
                    new_password = None;
                }

                match update_user_profile(&mut conn, user_id, new_username.as_deref(), new_password.as_deref()) {
                    Ok(_) => println!("User profile updated successfully."),
                    Err(e) => handle_error(e),
                }
            }
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action. Please try again."),
        }
    }
}
