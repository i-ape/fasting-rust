extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use std::io::{self, Write};

use crate::errors::handle_error;
use db::establish_connection;
use dotenv::dotenv;
use handlers::analytics::{
    get_fasting_history, calculate_average_fasting_duration, calculate_weekly_fasting_summary,
    calculate_current_streak, calculate_total_fasting_time,
};
use handlers::fasting::{start_fasting, stop_fasting};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

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
        let choice = prompt_input("\nChoose an action: register, login, associate, update, find, checkpoints, or exit: ").to_lowercase();

        match choice.as_str() {
            "register" => handle_register(&mut conn),
            "login" => handle_login(&mut conn),
            "associate" => handle_device_association(&mut conn),
            "update" => handle_update(&mut conn),
            "find" => handle_find(&mut conn),
            "checkpoints" => handle_checkpoints(&mut conn),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action. Please try again."),
        }
    }
}

/// Handles retrieving fasting checkpoints
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
                println!("No fasting checkpoints achieved yet.");
            } else {
                println!("Fasting checkpoints achieved: {:?}", checkpoints);
            }
        }
        Err(e) => handle_error(e),
    }
}


/// Handles fasting history retrieval
fn handle_history(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = prompt_input("Enter User ID: ").parse().unwrap_or(-1);
    match get_fasting_history(conn, user_id) {
        Ok(history) => println!("Fasting History: {:?}", history),
        Err(e) => handle_error(e),
    }
}

/// Handles average fasting duration calculation
fn handle_average(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = prompt_input("Enter User ID: ").parse().unwrap_or(-1);
    match calculate_average_fasting_duration(conn, user_id) {
        Ok(Some(avg)) => println!("Average Fasting Duration: {} minutes", avg),
        Ok(None) => println!("No fasting events found."),
        Err(e) => handle_error(e),
    }
}

/// Handles total fasting time calculation
fn handle_total(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = prompt_input("Enter User ID: ").parse().unwrap_or(-1);
    match calculate_total_fasting_time(conn, user_id) {
        Ok(total) => println!("Total Fasting Time: {} minutes", total),
        Err(e) => handle_error(e),
    }
}

/// Handles streak calculation
fn handle_streak(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = prompt_input("Enter User ID: ").parse().unwrap_or(-1);
    match calculate_current_streak(conn, user_id) {
        Ok(streak) => println!("Current Fasting Streak: {} days", streak),
        Err(e) => handle_error(e),
    }
}

/// Handles weekly fasting summary
fn handle_summary(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = prompt_input("Enter User ID: ").parse().unwrap_or(-1);
    let start_date: NaiveDateTime =
        prompt_input("Enter start date (YYYY-MM-DD HH:MM): ").parse().unwrap_or_default();
    let end_date: NaiveDateTime =
        prompt_input("Enter end date (YYYY-MM-DD HH:MM): ").parse().unwrap_or_default();

    match calculate_weekly_fasting_summary(conn, user_id, start_date, end_date) {
        Ok(total) => println!("Weekly Fasting Summary: {} minutes", total),
        Err(e) => handle_error(e),
    }
}
