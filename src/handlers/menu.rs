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
    loop {
        println!("\nAnalytics Menu:");
        println!("1. Fasting History");
        println!("2. Average Fasting Duration");
        println!("3. Current Streak");
        println!("4. Total Fasting Time");
        println!("5. Back to Main Menu");
    
        match prompt_user_choice("Enter your choice (1-5): ") {
            Some(1) => {
                let user_id = prompt_user_id(); // Prompt for the user ID
                if let Err(e) = show_fasting_history(conn) {
                    eprintln!("Error retrieving fasting history: {}", e);
                }
            }
            Some(2) => {
                let user_id = prompt_user_id();
                match calculate_average_fasting_duration(conn, user_id) {
                    Ok(Some(avg)) => println!("Average Fasting Duration: {} minutes", avg),
                    Ok(None) => println!("No fasting data available."),
                    Err(e) => eprintln!("Error calculating average fasting duration: {}", e),
                }
            }
            Some(3) => {
                let user_id = prompt_user_id();
                match calculate_current_streak(conn, user_id) {
                    Ok(streak) => println!("Current streak: {} days", streak),
                    Err(e) => eprintln!("Error calculating streak: {}", e),
                }
            }
            Some(4) => {
                let user_id = prompt_user_id();
                match calculate_total_fasting_time(conn, user_id) {
                    Ok(total) => println!("Total Fasting Time: {} minutes", total),
                    Err(e) => eprintln!("Error calculating total fasting time: {}", e),
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
