use chrono::Utc;
use diesel::SqliteConnection;
use std::io::{self, Write};

use crate::users::login::{
    login, associate_device_id
};


use crate::handlers::fasting::{
    get_current_fasting_status, start_fasting, stop_fasting, get_user_fasting_sessions, remove_fasting_goal, update_fasting_goal
};
use crate::handlers::analytics::{
    calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history,
};
use crate::handlers::goals::{add_goal, view_goals};
use crate::models::User;

/// Displays the main menu and handles user actions.
pub fn display_main_menu(conn: &mut SqliteConnection) {
    let user_id = prompt_user_id(); // ✅ Prompt once for user ID

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
            _ => println!("Invalid choice. Please select a valid option."),
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
        println!("5. View Goals");
        println!("6. Update Fasting Goal");
        println!("7. Remove Fasting Goal");
        println!("8. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-8): ") {
            Some(1) => {
                let goal_id = prompt_optional_goal_id();

                if let Err(e) = start_fasting(conn, user_id, Utc::now().naive_utc(), goal_id) {
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
            Some(5) => {
                if let Err(e) = view_goals(user_id, conn) {
                    eprintln!("Error viewing goals: {}", e);
                }
            }
            Some(6) => {  // ✅ NEW: Update fasting goal
                let new_goal_id = prompt_optional_goal_id();
                if let Err(e) = update_fasting_goal(conn, user_id, new_goal_id) {
                    eprintln!("Error updating fasting goal: {}", e);
                } else {
                    println!("Fasting goal updated successfully.");
                }
            }
            Some(7) => {  // ✅ NEW: Remove fasting goal
                if let Err(e) = remove_fasting_goal(conn, user_id) {
                    eprintln!("Error removing fasting goal: {}", e);
                } else {
                    println!("Fasting goal removed successfully.");
                }
            }
            Some(8) => break,
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
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

/// Handles the analytics-related menu actions.
fn handle_analytics_menu(conn: &mut SqliteConnection, user_id: i32) {
    loop {
        println!("\nAnalytics Menu:");
        println!("1. Fasting History");
        println!("2. Average Fasting Duration");
        println!("3. Total Fasting Time");
        println!("4. View All Fasting Sessions");
        println!("5. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-5): ") {
            Some(1) => show_fasting_history(conn, user_id),
            Some(2) => match calculate_average_fasting_duration(conn, user_id) {
                Ok(Some(avg)) => println!("Average Fasting Duration: {} minutes.", avg),
                Ok(_none) => println!("No fasting data available."),
                Err(e) => eprintln!("Error calculating average fasting duration: {}", e),
            },
            Some(3) => match calculate_total_fasting_time(conn, user_id) {
                Ok(total) => println!("Total Fasting Time: {} minutes.", total),
                Err(e) => eprintln!("Error calculating total fasting time: {}", e),
            },
            Some(4) => match get_user_fasting_sessions(conn, user_id) {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        println!("No fasting sessions found for user {}.", user_id);
                    } else {
                        println!("Fasting sessions for user {}:", user_id);
                        for session in sessions {
                            println!(
                                "- Start: {}, End: {:?}",
                                session.start_time,
                                session.stop_time.unwrap_or_else(|| Utc::now().naive_utc())
                            );
                        }
                    }
                }
                Err(e) => eprintln!("Error retrieving fasting sessions: {}", e),
            },
            Some(5) => break,
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
/// Handles user login and authentication.
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
                        println!("Login successful! Welcome, {}.", user.username);
                        return Some(user); // ✅ Return user object after login
                    }
                    Err(e) => eprintln!("Login failed: {}", e),
                }
            }
            Some(2) => {
                let device_id = prompt_user_input("Enter your device ID: ");

                match login(conn, None, None, Some(&device_id)) {
                    Ok(user) => {
                        println!("Device login successful! Welcome, {}.", user.username);
                        return Some(user); // ✅ Return user object
                    }
                    Err(e) => eprintln!("Login failed: {}", e),
                }
            }
            Some(3) => return None, // Go back to main menu
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}
/// Allows the user to update their account settings, including linking a device ID.
fn handle_account_settings(conn: &mut SqliteConnection, user: &User) {
    loop {
        println!("\nAccount Settings:");
        println!("1. Link a New Device ID");
        println!("2. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-2): ") {
            Some(1) => {
                let new_device_id = prompt_user_input("Enter your new device ID: ");

                match associate_device_id(conn, user.id, &new_device_id) {
                    Ok(_) => println!("Device ID linked successfully."),
                    Err(e) => eprintln!("Failed to link device ID: {}", e),
                }
            }
            Some(2) => break,
            _ => println!("Invalid choice. Please select a valid option."),
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
