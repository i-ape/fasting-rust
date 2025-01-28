use chrono::Utc;
use diesel::SqliteConnection;
use crate::handlers::analytics::{
    show_fasting_history, calculate_average_fasting_duration, calculate_current_streak,
    calculate_total_fasting_time,
};
use crate::handlers::fasting::{start_fasting, stop_fasting, get_current_fasting_status};
use crate::handlers::goals::{add_goal, view_goals};

/// Displays the main menu and handles user actions.
pub fn display_main_menu(conn: &mut SqliteConnection) {
    loop {
        println!("\nMain Menu:");
        println!("1. Fasting Menu");
        println!("2. Analytics Menu");
        println!("3. Exit");

        match prompt_user_choice("Enter your choice (1-3): ") {
            Some(1) => handle_fasting_menu(conn),
            Some(2) => handle_analytics_menu(conn),
            Some(3) => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option (1-3)."),
        }
    }
}

/// Handles the fasting-related menu actions.
pub fn handle_fasting_menu(conn: &mut SqliteConnection) {
    // Prompt for user ID once at the beginning
    let user_id = prompt_user_id();

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
                    eprintln!("Error starting fasting session for user {}: {}", user_id, e);
                } else {
                    println!("Fasting session started successfully for user {}.", user_id);
                }
            }
            Some(2) => {
                if let Err(e) = stop_fasting(conn, user_id, Utc::now().naive_utc()) {
                    eprintln!("Error stopping fasting session for user {}: {}", user_id, e);
                } else {
                    println!("Fasting session stopped successfully for user {}.", user_id);
                }
            }
            Some(3) => {
                match get_current_fasting_status(conn, user_id) {
                    Ok(Some((start_time, duration))) => println!(
                        "User {}: Fasting started at {} and has lasted for {} minutes.",
                        user_id, start_time, duration
                    ),
                    Ok(None) => println!("No active fasting session found for user {}.", user_id),
                    Err(e) => eprintln!("Error retrieving fasting status for user {}: {}", user_id, e),
                }
            }
            Some(4) => {
                if let Err(e) = add_goal(user_id, conn) {
                    eprintln!("Error adding goal for user {}: {}", user_id, e);
                } else {
                    println!("Goal added successfully for user {}.", user_id);
                }
            }
            Some(5) => break,
            _ => println!("Invalid choice. Please select a valid option (1-5)."),
        }
    }
}

/// Handles the analytics-related menu actions.
pub fn handle_analytics_menu(conn: &mut SqliteConnection) {
    // Prompt for user ID once at the beginning
    let user_id = prompt_user_id();

    loop {
        println!("\nAnalytics Menu:");
        println!("1. Fasting History");
        println!("2. Average Fasting Duration");
        println!("3. Current Streak");
        println!("4. Total Fasting Time");
        println!("5. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-5): ") {
            Some(1) => {
                show_fasting_history(conn, user_id);
            }
            Some(2) => {
                match calculate_average_fasting_duration(conn, user_id) {
                    Ok(Some(avg)) => println!("User {}: Average Fasting Duration: {} minutes.", user_id, avg),
                    Ok(None) => println!("No fasting data available for user {}.", user_id),
                    Err(e) => eprintln!("Error calculating average fasting duration for user {}: {}", user_id, e),
                }
            }
            Some(3) => {
                match calculate_current_streak(conn, user_id) {
                    Ok(streak) => println!("User {}: Current streak is {} days.", user_id, streak),
                    Err(e) => eprintln!("Error calculating streak for user {}: {}", user_id, e),
                }
            }
            Some(4) => {
                match calculate_total_fasting_time(conn, user_id) {
                    Ok(total) => println!("User {}: Total Fasting Time: {} minutes.", user_id, total),
                    Err(e) => eprintln!("Error calculating total fasting time for user {}: {}", user_id, e),
                }
            }
            Some(5) => break,
            _ => println!("Invalid choice. Please select a valid option (1-5)."),
        }
    }
}

/// Prompts the user for a menu choice and returns their selection.
fn prompt_user_choice(message: &str) -> Option<i32> {
    use std::io::{self, Write};

    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().ok()
}

/// Prompts the user to input their user ID.
fn prompt_user_id() -> i32 {
    use std::io::{self, Write};

    print!("Enter your user ID: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().unwrap_or_else(|_| {
        println!("Invalid input. Defaulting user ID to 1.");
        1
    })
}
