use chrono::Utc;
use diesel::SqliteConnection;
use crate::handlers::analytics::{
    show_fasting_history, calculate_average_fasting_duration, calculate_total_fasting_time,
};
use crate::handlers::fasting::{start_fasting, stop_fasting, get_current_fasting_status};
use crate::handlers::goals::add_goal;
use std::io::{self, Write};

/// Displays the main menu and handles user actions.
pub fn display_main_menu(conn: &mut SqliteConnection) {
    let user_id = prompt_user_id(); // Prompt once for user ID

    loop {
        println!("\nMain Menu:");
        println!("1. Fasting Menu");
        println!("2. Analytics Menu");
        println!("3. Exit");

        match prompt_user_choice("Enter your choice (1-3): ") {
            Some(1) => handle_fasting_menu(conn, user_id),
            Some(2) => handle_analytics_menu(conn, user_id),
            Some(3) => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option (1-3)."),
        }
    }
}

/// Handles the fasting-related menu actions.
fn handle_fasting_menu(conn: &mut SqliteConnection, user_id: i32) {
    loop {
        println!("\nFasting Menu:");
        println!("1. Start Fasting");
        println!("2. Stop Fasting");
        println!("3. Fasting Status");
        println!("4. Add Goal");
        println!("5. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-5): ") {
            Some(1) => {
                if let Err(e) = start_fasting(conn, user_id, Utc::now().naive_utc()) {
                    eprintln!("Error starting fasting session: {}", e);
                } else {
                    println!("Fasting session started successfully.");
                }
            }
            Some(2) => {
                if let Err(e) = stop_fasting(conn, user_id, Utc::now().naive_utc()) {
                    eprintln!("Error stopping fasting session: {}", e);
                } else {
                    println!("Fasting session stopped successfully.");
                }
            }
            Some(3) => match get_current_fasting_status(conn, user_id) {
                Ok(Some((start_time, duration))) => println!(
                    "Fasting started at {} and has lasted for {} minutes.",
                    start_time, duration
                ),
                Ok(None) => println!("No active fasting session found."),
                Err(e) => eprintln!("Error retrieving fasting status: {}", e),
            },
            Some(4) => {
                if let Err(e) = add_goal(user_id, conn) {
                    eprintln!("Error adding goal: {}", e);
                } else {
                    println!("Goal added successfully.");
                }
            }
            Some(5) => break,
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

/// Handles the analytics-related menu actions.
fn handle_analytics_menu(conn: &mut SqliteConnection, user_id: i32) {
    loop {
        println!("\nAnalytics Menu:");
        println!("1. Fasting History");
        println!("2. Average Fasting Duration");
        println!("3. Total Fasting Time");
        println!("4. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-4): ") {
            Some(1) => show_fasting_history(conn, user_id),
            Some(2) => match calculate_average_fasting_duration(conn, user_id) {
                Ok(Some(avg)) => println!("Average Fasting Duration: {} minutes.", avg),
                Ok(None) => println!("No fasting data available."),
                Err(e) => eprintln!("Error calculating average fasting duration: {}", e),
            },
            Some(3) => match calculate_total_fasting_time(conn, user_id) {
                Ok(total) => println!("Total Fasting Time: {} minutes.", total),
                Err(e) => eprintln!("Error calculating total fasting time: {}", e),
            },
            Some(4) => break,
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

/// Prompts the user for a menu choice and returns their selection.
fn prompt_user_choice(message: &str) -> Option<i32> {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().ok()
}

/// Prompts the user to input their user ID.
fn prompt_user_id() -> i32 {
    print!("Enter your user ID: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid input. Defaulting user ID to 1.");
            1
        }
    }
}
