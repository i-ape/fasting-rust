extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use std::io::{self, Write};

use crate::errors::handle_error;
use db::establish_connection;
use dotenv::dotenv;
use handlers::fasting::{start_fasting, stop_fasting};
use handlers::analytics::{
    get_fasting_history, calculate_average_fasting_duration, calculate_weekly_fasting_summary,
    calculate_current_streak,
};
use crate::users::{login_user_or_device, update_user_profile, register_user};

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

/// The main entry point for the application.
fn main() {
    dotenv().ok();

    // Establish a database connection
    let mut conn = match establish_connection() {
        Ok(connection) => connection,
        Err(e) => {
            handle_error(e);
            return;
        }
    };

    // Main program loop
    loop {
        println!("\n=== Fasting App ===");
        let choice = prompt_input(
            "\nChoose an action: register, login, update, history, summary, streak, or exit: ",
        )
        .to_lowercase();

        match choice.as_str() {
            "register" => handle_register(&mut conn),
            "login" => handle_login(&mut conn),
            "update" => handle_update(&mut conn),
            "history" => handle_fasting_history(&mut conn),
            "summary" => handle_fasting_summary(&mut conn),
            "streak" => handle_fasting_streak(&mut conn),
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

/// Handles user login, supporting both username/password and device ID.
fn handle_login(conn: &mut diesel::SqliteConnection) {
    let login_method = prompt_input("Login with: username or device? (Enter 'username' or 'device'): ").to_lowercase();

    match login_method.as_str() {
        "username" => {
            let username = prompt_input("Enter your username: ");
            let password = prompt_input("Enter your password: ");

            match login_user_or_device(conn, Some(&username), Some(&password), None) {
                Ok(user) => {
                    if let Some(id) = user.id {
                        println!("Login successful. User ID: {}", id);
                        handle_fasting_session(conn, id);
                    } else {
                        println!("Login successful, but User ID is not available.");
                    }
                }
                Err(e) => handle_error(e),
            }
        }
        "device" => {
            let device_id = prompt_input("Enter your device ID: ");

            match login_user_or_device(conn, None, None, Some(&device_id)) {
                Ok(user) => {
                    if let Some(id) = user.id {
                        println!("Login successful using device ID. User ID: {}", id);
                        handle_fasting_session(conn, id);
                    } else {
                        println!("Login successful using device ID, but User ID is not available.");
                    }
                }
                Err(e) => handle_error(e),
            }
        }
        _ => println!("Invalid login method. Please try again."),
    }
}

/// Handles user profile updates (username, password, or device ID).
fn handle_update(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) if id >= 0 => id,
        _ => {
            println!("Invalid User ID. Please enter a positive integer.");
            return;
        }
    };

    let mut new_username = Some(prompt_input("Enter a new username (or press Enter to skip): "));
    let mut new_password = Some(prompt_input("Enter a new password (or press Enter to skip): "));
    let mut new_device_id = Some(prompt_input("Enter a new device ID (or press Enter to skip): "));

    if new_username.as_deref() == Some("") {
        new_username = None;
    }
    if new_password.as_deref() == Some("") {
        new_password = None;
    }
    if new_device_id.as_deref() == Some("") {
        new_device_id = None;
    }

    match update_user_profile(
        conn,
        user_id,
        new_username.as_deref(),
        new_password.as_deref(),
        new_device_id.as_deref(),
    ) {
        Ok(_) => println!("User profile updated successfully."),
        Err(e) => handle_error(e),
    }
}

/// Handles viewing a user's fasting history.
fn handle_fasting_history(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid User ID.");
            return;
        }
    };

    match get_fasting_history(conn, user_id) {
        Ok(events) => {
            println!("Fasting history:");
            for event in events {
                println!("{:?}", event);
            }
        }
        Err(e) => handle_error(e),
    }
}

/// Handles calculating a user's average fasting duration.
fn handle_fasting_summary(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid User ID.");
            return;
        }
    };

    match calculate_average_fasting_duration(conn, user_id) {
        Ok(Some(avg)) => println!("Your average fasting duration is {} minutes.", avg),
        Ok(None) => println!("No fasting history found."),
        Err(e) => handle_error(e),
    }
}

/// Handles calculating a user's current fasting streak.
fn handle_fasting_streak(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid User ID.");
            return;
        }
    };

    match calculate_current_streak(conn, user_id) {
        Ok(streak) => println!("Your current fasting streak is {} days.", streak),
        Err(e) => handle_error(e),
    }
}

/// Handles a user's fasting session (start and stop).
fn handle_fasting_session(conn: &mut diesel::SqliteConnection, user_id: i32) {
    match start_fasting(conn, user_id, chrono::Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session started."),
        Err(e) => handle_error(e),
    }

    match stop_fasting(conn, user_id, chrono::Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session stopped."),
        Err(e) => handle_error(e),
    }
}
