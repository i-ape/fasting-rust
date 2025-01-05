use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use crate::models::FastingGoal;
use crate::handlers::fasting::{start_fasting, stop_fasting, get_current_fasting_status};
use crate::handlers::analytics::{
    show_fasting_history, calculate_average_fasting_duration, calculate_current_streak, calculate_total_fasting_time,
};
use crate::handlers::goals::{add_goal, view_goals};

/// Displays the main menu and handles user actions.
pub fn display_main_menu(conn: &mut SqliteConnection) {
    loop {
        println!("\nMain Menu:");
        println!("1. Fasting Menu");
        println!("2. Analytics Menu");
        println!("3. Exit");

        match prompt_user_choice("Enter your choice: ") {
            Some(1) => handle_fasting_menu(conn),
            Some(2) => handle_analytics_menu(conn),
            Some(3) => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

/// Handles the fasting-related menu actions.
pub fn handle_fasting_menu(conn: &mut SqliteConnection) {
    loop {
        println!("\nFasting Menu:");
        println!("1. Start Fasting");
        println!("2. Stop Fasting");
        println!("3. Fasting Status");
        println!("4. Add Goal");
        println!("5. Back to Main Menu");

        match prompt_user_choice("Enter your choice: ") {
            Some(1) => {
                let user_id = prompt_user_id(); // Prompt for user ID
                let event_start_time = Utc::now().naive_utc(); // Get the current time
                match start_fasting(conn, user_id, event_start_time) {
                    Ok(_) => println!("Fasting session started successfully."),
                    Err(e) => println!("Error starting fasting session: {:?}", e),
                }
            }
            Some(2) => match stop_fasting(conn) {
                Ok(_) => println!("Fasting session stopped successfully."),
                Err(e) => println!("Error stopping fasting session: {:?}", e),
            },
            Some(3) => match get_current_fasting_status(conn) {
                Ok(Some((start_time, duration))) => println!(
                    "Fasting started at: {} and has lasted for {} minutes.",
                    start_time, duration
                ),
                Ok(None) => println!("No active fasting session."),
                Err(e) => println!("Error retrieving fasting status: {:?}", e),
            },
            Some(4) => match add_goal(conn) {
                Ok(_) => println!("Goal added successfully."),
                Err(e) => println!("Error adding goal: {:?}", e),
            },
            Some(5) => break,
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

/// Handles the analytics-related menu actions.
pub fn handle_analytics_menu(conn: &mut SqliteConnection) {
    loop {
        println!("\nAnalytics Menu:");
        println!("1. Fasting History");
        println!("2. Average Fasting Duration");
        println!("3. Current Streak");
        println!("4. Total Fasting Time");
        println!("5. Back to Main Menu");

        match prompt_user_choice("Enter your choice: ") {
            Some(1) => match show_fasting_history(conn) {
                Ok(_) => (),
                Err(e) => println!("Error retrieving fasting history: {:?}", e),
            },
            Some(2) => match calculate_average_fasting_duration(conn) {
                Ok(Some(avg)) => println!("Average Fasting Duration: {} minutes", avg),
                Ok(None) => println!("No fasting data available."),
                Err(e) => println!("Error calculating average fasting duration: {:?}", e),
            },
            Some(3) => match calculate_current_streak(conn) {
                Ok(streak) => println!("Current streak: {} days", streak),
                Err(e) => println!("Error calculating streak: {:?}", e),
            },
            Some(4) => match calculate_total_fasting_time(conn) {
                Ok(total) => println!("Total Fasting Time: {} minutes", total),
                Err(e) => println!("Error calculating total fasting time: {:?}", e),
            },
            Some(5) => break,
            _ => println!("Invalid choice. Please select a valid option."),
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
