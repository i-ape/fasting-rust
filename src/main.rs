extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use std::io::{self, Write};

use dotenv::dotenv;

use crate::db::establish_connection;
use crate::errors::handle_error;
use crate::handlers::analytics::{
    calculate_average_fasting_duration, calculate_current_streak, calculate_total_fasting_time,
    calculate_weekly_fasting_summary, get_fasting_checkpoints, get_fasting_history,
};
use crate::handlers::fasting::{start_fasting, stop_fasting};
use crate::users::{register_user, login_user, update_user_profile};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod users;

/// Prompts the user for input with a given message.
fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Main entry point
fn main() {
    dotenv().ok();

    let mut conn = match establish_connection() {
        Ok(connection) => connection,
        Err(e) => {
            handle_error(e);
            return;
        }
    };

    loop {
        let choice = prompt_input("\nChoose an action: register, login, update, checkpoints, or exit: ").to_lowercase();

        match choice.as_str() {
            "register" => handle_register(&mut conn),
            "login" => handle_login(&mut conn),
            "update" => handle_update(&mut conn),
            "checkpoints" => handle_checkpoints(&mut conn),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action. Please try again."),
        }
    }
}

/// Handles user registration.
fn handle_register(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input("Enter a new username: ");
    let password = prompt_input("Enter a new password: ");

    match register_user(conn, &username, &password) {
        Ok(_) => println!("User registered successfully."),
        Err(e) => handle_error(e),
    }
}

/// Handles user login.
fn handle_login(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input("Enter your username: ");
    let password = prompt_input("Enter your password: ");

    match login_user(conn, &username, &password) {
        Ok(user) => {
            println!("Login successful: {:?}", user);
        }
        Err(e) => handle_error(e),
    }
}

/// Handles updating user profiles.
fn handle_update(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid User ID.");
            return;
        }
    };

    let new_username = Some(prompt_input("Enter a new username (or press Enter to skip): "));
    let new_password = Some(prompt_input("Enter a new password (or press Enter to skip): "));

    match update_user_profile(conn, user_id, new_username.as_deref(), new_password.as_deref()) {
        Ok(_) => println!("User profile updated successfully."),
        Err(e) => handle_error(e),
    }
}

/// Handles fasting checkpoints.
fn handle_checkpoints(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid User ID.");
            return;
        }
    };

    match get_fasting_checkpoints(conn, user_id) {
        Ok(checkpoints) => {
            if checkpoints.is_empty() {
                println!("No fasting checkpoints achieved.");
            } else {
                println!("Achieved fasting checkpoints: {:?}", checkpoints);
            }
        }
        Err(e) => handle_error(e),
    }
}
