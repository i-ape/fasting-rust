use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use crate::handlers::fasting::{start_fasting, stop_fasting, check_fasting_status};
use crate::handlers::analytics::{
    show_fasting_history, show_average_fasting_duration, show_current_streak, show_total_fasting_time, view_goals,
};
use crate::handlers::goals::add_goal;

/// Displays the main menu and handles user actions.
pub fn display_main_menu(conn: &mut SqliteConnection) {
    loop {
        println!("\nMain Menu:");
        println!("1. Fasting Menu");
        println!("2. Analytics Menu");
        println!("3. Exit");

        let choice = prompt_user_choice();

        match choice {
            1 => handle_fasting_menu(conn),
            2 => handle_analytics_menu(conn),
            3 => {
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

        let choice = prompt_user_choice();

        match choice {
            1 => {
                let user_id = prompt_user_id(); // Prompt for user ID
                let event_start_time = Utc::now().naive_utc(); // Get the current time
                start_fasting(conn, user_id, event_start_time); // Pass all arguments
            }
            2 => stop_fasting(conn),
            3 => check_fasting_status(conn),
            4 => add_goal(conn),
            5 => break,
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

        let choice = prompt_user_choice();

        match choice {
            1 => show_fasting_history(conn),
            2 => show_average_fasting_duration(conn),
            3 => show_current_streak(conn),
            4 => show_total_fasting_time(conn),
            5 => break,
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

/// Prompts the user for a menu choice and returns their selection.
fn prompt_user_choice() -> i32 {
    use std::io::{self, Write};

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().unwrap_or(-1) // Return -1 for invalid input
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
