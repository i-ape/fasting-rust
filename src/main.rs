extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use std::io::{self, Write};

use crate::errors::handle_error;
use db::establish_connection;
use dotenv::dotenv;
use handlers::fasting::manage_fasting_session;
use crate::users::{register_user, login_user, login_user_or_device, associate_device_id, update_user_profile};

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
        let choice = prompt_input("\nChoose an action: register, login, associate, update, or exit: ").to_lowercase();

        match choice.as_str() {
            "register" => handle_register(&mut conn),
            "login" => handle_login(&mut conn),
            "associate" => handle_device_association(&mut conn),
            "update" => handle_update(&mut conn),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action. Please try again."),
        }
    }
}

/// Handles the user registration process.
fn handle_register(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input("Enter a new username: ");
    let password = prompt_input("Enter a new password: ");

    match register_user(conn, &username, &password) {
        Ok(_) => println!("User registered successfully."),
        Err(e) => handle_error(e),
    }
}

/// Handles the user login process, supporting both username/password and device ID login.
fn handle_login(conn: &mut diesel::SqliteConnection) {
    let login_method = prompt_input("Login with: username or device? (Enter 'username' or 'device'): ").to_lowercase();

    match login_method.as_str() {
        "username" => {
            let username = prompt_input("Enter your username: ");
            let password = prompt_input("Enter your password: ");

            match login_user(conn, &username, &password) {
                Ok(user) => {
                    if let Some(id) = user.id {
                        println!("Login successful. User ID: {}", id);
                        manage_fasting_session(conn, id);
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
                        manage_fasting_session(conn, id);
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

/// Handles the process of associating a device ID with a user account.
fn handle_device_association(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) if id >= 0 => id,
        _ => {
            println!("Invalid User ID. Please enter a positive integer.");
            return;
        }
    };

    let device_id = prompt_input("Enter your device ID to associate: ");

    match associate_device_id(conn, user_id, &device_id) {
        Ok(_) => println!("Device ID associated successfully."),
        Err(e) => handle_error(e),
    }
}

/// Handles the process of updating user profile details.
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

    if new_username.as_deref() == Some("") {
        new_username = None;
    }
    if new_password.as_deref() == Some("") {
        new_password = None;
    }

    match update_user_profile(conn, user_id, new_username.as_deref(), new_password.as_deref()) {
        Ok(_) => println!("User profile updated successfully."),
        Err(e) => handle_error(e),
    }
}
