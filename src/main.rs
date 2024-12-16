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

/// Generic function to prompt input.
fn prompt_input<T: std::str::FromStr>(message: &str) -> Option<T> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok()
}

/// Retrieves and validates a user ID.
fn get_valid_user_id() -> Option<i32> {
    prompt_input::<i32>("Enter your User ID: ").filter(|id| *id > 0)
}

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
        let choice = prompt_input::<String>("\nChoose an action: register, login, update, fasting, analytics, or exit: ")
            .unwrap_or_else(|| "invalid".to_string())
            .to_lowercase();

        match choice.as_str() {
            "register" => handle_register(&mut conn),
            "login" => handle_login(&mut conn),
            "update" => handle_update(&mut conn),
            "fasting" => handle_fasting_menu(&mut conn),
            "analytics" => handle_analytics_menu(&mut conn),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action. Please try again."),
        }
    }
}

// Simplified handler examples
fn handle_register(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input::<String>("Enter a new username: ").unwrap_or_default();
    let password = prompt_input::<String>("Enter a new password: ").unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        println!("Username and password cannot be empty.");
        return;
    }

    match register_user(conn, &username, &password) {
        Ok(_) => println!("User registered successfully."),
        Err(e) => handle_error(e),
    }
}

fn handle_login(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input::<String>("Enter your username: ").unwrap_or_default();
    let password = prompt_input::<String>("Enter your password: ").unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        println!("Username and password cannot be empty.");
        return;
    }

    match login_user(conn, &username, &password) {
        Ok(user) => println!("Login successful: {:?}", user),
        Err(e) => handle_error(e),
    }
}

fn handle_update(conn: &mut diesel::SqliteConnection) {
    let user_id = match get_valid_user_id() {
        Some(id) => id,
        None => {
            println!("Invalid User ID.");
            return;
        }
    };

    let new_username = prompt_input::<String>("Enter a new username (or press Enter to skip): ");
    let new_password = prompt_input::<String>("Enter a new password (or press Enter to skip): ");
    let new_device_id = prompt_input::<String>("Enter a new device ID (or press Enter to skip): ");

    match update_user_profile(
        conn,
        user_id,
        new_username.filter(|s| !s.is_empty()).as_deref(),
        new_password.filter(|s| !s.is_empty()).as_deref(),
        new_device_id.filter(|s| !s.is_empty()).as_deref(),
    ) {
        Ok(_) => println!("User profile updated successfully."),
        Err(e) => handle_error(e),
    }
}
