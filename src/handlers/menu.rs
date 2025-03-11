use chrono::Utc;
use diesel::SqliteConnection;
use std::io::{self, Write};

use crate::handlers::fasting::{
    get_current_fasting_status, start_fasting, stop_fasting, get_user_fasting_sessions,remove_fasting_goal, update_fasting_goal
};
use crate::handlers::analytics::{
    calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history,
};
use crate::handlers::goals::{add_goal, view_goals};

use crate::users::login::{login, associate_device_id};
use crate::models::User;

/// ✅ Displays the main menu **AFTER login**.
pub fn display_main_menu(conn: &mut SqliteConnection) {
    println!("Welcome to Fasting Tracker!");

    // ✅ Require user login first
    let user = match handle_login_menu(conn) {
        Some(user) => user,  // ✅ Proceed with authenticated user
        None => {
            println!("❌ Login required. Exiting...");
            return;
        }
    };

    loop {
        println!("\nMain Menu:");
        println!("1. Fasting Menu");
        println!("2. Analytics Menu");
        println!("3. Account Settings");  // ✅ New option for linking device
        println!("4. Exit");

        match prompt_user_choice("Enter your choice (1-4): ") {
            Some(1) => handle_fasting_menu(conn, &user),
            Some(2) => handle_analytics_menu(conn, &user),
            Some(3) => handle_account_settings(conn, &user), // ✅ Link a device
            Some(4) => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice. Please select a valid option."),
        }
    }
}

/// ✅ Handles user login and authentication.
fn handle_login_menu(conn: &mut SqliteConnection) -> Option<User> {
    loop {
        println!("\nLogin Menu:");
        println!("1. Login with Username & Password");
        println!("2. Login with Device ID");
        println!("3. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-3): ") {
            Some(1) => {
                let username = prompt_user_input("Enter your username: ");
                let password = prompt_user_input("Enter your password: ");

                match login(conn, Some(&username), Some(&password), None) {
                    Ok(user) => {
                        println!("✅ Login successful! Welcome, {}.", user.username);
                        return Some(user);
                    }
                    Err(e) => eprintln!("❌ Login failed: {}", e),
                }
            }
            Some(2) => {
                let device_id = prompt_user_input("Enter your device ID: ");

                match login(conn, None, None, Some(&device_id)) {
                    Ok(user) => {
                        println!("✅ Device login successful! Welcome, {}.", user.username);
                        return Some(user);
                    }
                    Err(e) => eprintln!("❌ Login failed: {}", e),
                }
            }
            Some(3) => return None, // Go back to main menu
            _ => println!("❌ Invalid choice. Please select a valid option."),
        }
    }
}

/// ✅ Handles account settings (Linking a device ID).
fn handle_account_settings(conn: &mut SqliteConnection, user: &User) {
    loop {
        println!("\nAccount Settings:");
        println!("1. Link a New Device ID");
        println!("2. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-2): ") {
            Some(1) => {
                let new_device_id = prompt_user_input("Enter your new device ID: ");

                match associate_device_id(conn, user.id, &new_device_id) {
                    Ok(_) => println!("✅ Device ID linked successfully."),
                    Err(e) => eprintln!("❌ Failed to link device ID: {}", e),
                }
            }
            Some(2) => break,
            _ => println!("❌ Invalid choice. Please select a valid option."),
        }
    }
}

/// ✅ Prompts the user for input and returns the trimmed string.
fn prompt_user_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// ✅ Prompts the user for a menu choice and returns their selection.
fn prompt_user_choice(message: &str) -> Option<i32> {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().ok()
}

/// ✅ Prompts the user for a goal ID (Optional)
fn prompt_optional_goal_id() -> Option<i32> {
    print!("Enter Goal ID (or press Enter to skip): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<i32>() {
        Ok(id) => Some(id), // ✅ User entered a valid goal ID
        Err(_) => None,     // ✅ User skipped (pressed Enter)
    }
}
