extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use std::io::{self, Write};

use chrono::NaiveDateTime;
use dotenv::dotenv;

use crate::db::establish_connection;
use crate::errors::handle_error;
use crate::handlers::analytics::{
    calculate_average_fasting_duration, calculate_current_streak, calculate_total_fasting_time,
    calculate_weekly_fasting_summary, get_fasting_checkpoints, get_fasting_history,
};
use crate::handlers::fasting::{start_fasting, stop_fasting, get_current_fasting_status};
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

/// Main function providing the primary menu to the user.
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
        let choice = prompt_input::<String>(
            "\nChoose an action: register, login, update, fasting, analytics, or exit: ",
        )
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

/// Handles user registration.
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

/// Handles user login.
fn handle_login(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input::<String>("Enter your username: ").unwrap_or_default();
    let password = prompt_input::<String>("Enter your password: ").unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        println!("Username and password cannot be empty.");
        return;
    }

    match login_user(conn, &username, &password) {
        Ok(user) => {
            println!("Login successful for user ID: {:?}", user.id);
            handle_fasting_menu(conn);
        }
        Err(e) => handle_error(e),
    }
}

/// Handles updating user profiles.
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

/// Handles the fasting-related actions.
fn handle_fasting_menu(conn: &mut diesel::SqliteConnection) {
    let user_id = match get_valid_user_id() {
        Some(id) => id,
        None => {
            println!("Invalid User ID.");
            return;
        }
    };

    loop {
        println!("\nFasting Menu: start, stop, status, back");
        let choice = prompt_input::<String>("Choose an action: ")
            .unwrap_or_default()
            .to_lowercase();

        match choice.as_str() {
            "start" => match start_fasting(conn, user_id, chrono::Utc::now().naive_utc()) {
                Ok(_) => println!("Fasting session started."),
                Err(e) => handle_error(e),
            },
            "stop" => match stop_fasting(conn, user_id, chrono::Utc::now().naive_utc()) {
                Ok(_) => println!("Fasting session stopped."),
                Err(e) => handle_error(e),
            },
            "status" => match get_current_fasting_status(conn, user_id) {
                Ok(Some((start, duration))) => {
                    println!(
                        "Current fasting started at: {}, duration: {} minutes.",
                        start, duration
                    );
                }
                Ok(None) => println!("No active fasting session."),
                Err(e) => handle_error(e),
            },
            "back" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

/// Handles analytics-related actions.
fn handle_analytics_menu(conn: &mut diesel::SqliteConnection) {
    let user_id = match get_valid_user_id() {
        Some(id) => id,
        None => {
            println!("Invalid User ID.");
            return;
        }
    };

    loop {
        println!("\nAnalytics Menu: history, average, streak, total, checkpoints, summary, back");
        let choice = prompt_input::<String>("Choose an action: ")
            .unwrap_or_default()
            .to_lowercase();

        match choice.as_str() {
            "history" => match get_fasting_history(conn, user_id) {
                Ok(events) => println!("Fasting History: {:?}", events),
                Err(e) => handle_error(e),
            },
            "average" => match calculate_average_fasting_duration(conn, user_id) {
                Ok(Some(avg)) => println!("Average Fasting Duration: {} minutes", avg),
                Ok(None) => println!("No fasting events found."),
                Err(e) => handle_error(e),
            },
            "streak" => match calculate_current_streak(conn, user_id) {
                Ok(streak) => println!("Current Streak: {} days", streak),
                Err(e) => handle_error(e),
            },
            "total" => match calculate_total_fasting_time(conn, user_id) {
                Ok(total) => println!("Total Fasting Time: {} minutes", total),
                Err(e) => handle_error(e),
            },
            "checkpoints" => match get_fasting_checkpoints(conn, user_id) {
                Ok(checkpoints) => println!("Achieved Checkpoints: {:?}", checkpoints),
                Err(e) => handle_error(e),
            },
            "summary" => {
                let start_date = prompt_input::<String>("Enter start date (YYYY-MM-DD HH:MM): ")
                    .and_then(|d| NaiveDateTime::parse_from_str(&d, "%Y-%m-%d %H:%M").ok());
                let end_date = prompt_input::<String>("Enter end date (YYYY-MM-DD HH:MM): ")
                    .and_then(|d| NaiveDateTime::parse_from_str(&d, "%Y-%m-%d %H:%M").ok());

                if let (Some(start), Some(end)) = (start_date, end_date) {
                    match calculate_weekly_fasting_summary(conn, user_id, start, end) {
                        Ok(total) => println!("Total Fasting Duration: {} minutes", total),
                        Err(e) => handle_error(e),
                    }
                } else {
                    println!("Invalid date format.");
                }
            }
            "back" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
