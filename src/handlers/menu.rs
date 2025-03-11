use chrono::Utc;
use diesel::SqliteConnection;
use std::io::{self, Write};

use crate::handlers::fasting::{
    get_current_fasting_status, start_fasting, stop_fasting, get_user_fasting_sessions, 
    remove_fasting_goal, update_fasting_goal
};
use crate::handlers::analytics::{
    calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history,
};
use crate::handlers::goals::{add_goal, view_goals};

use crate::users::login::{login, associate_device_id};
use crate::models::User; // ✅ Import User model

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

/// ✅ Handles the fasting-related menu actions.
fn handle_fasting_menu(conn: &mut SqliteConnection, user: &User) {
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
                if let Err(e) = start_fasting(conn, user.id, Utc::now().naive_utc(), goal_id) {
                    eprintln!("❌ Error starting fasting session: {}", e);
                } else {
                    println!("✅ Fasting session started successfully.");
                }
            }
            Some(2) => {
                if let Err(e) = stop_fasting(conn, user.id, Utc::now().naive_utc()) {
                    eprintln!("❌ Error stopping fasting session: {}", e);
                } else {
                    println!("✅ Fasting session stopped successfully.");
                }
            }
            Some(3) => match get_current_fasting_status(conn, user.id) {
                Ok(Some((start_time, duration))) => println!(
                    "⏳ Fasting started at {} and has lasted for {} minutes.",
                    start_time, duration
                ),
                Ok(None) => println!("❌ No active fasting session found."),
                Err(e) => eprintln!("❌ Error retrieving fasting status: {}", e),
            },
            Some(4) => {
                if let Err(e) = add_goal(user.id, conn) {
                    eprintln!("❌ Error adding goal: {}", e);
                } else {
                    println!("✅ Goal added successfully.");
                }
            }
            Some(5) => {
                if let Err(e) = view_goals(user.id, conn) {
                    eprintln!("❌ Error viewing goals: {}", e);
                }
            }
            Some(6) => {
                let new_goal_id = prompt_optional_goal_id();
                if let Err(e) = update_fasting_goal(conn, user.id, new_goal_id) {
                    eprintln!("❌ Error updating fasting goal: {}", e);
                } else {
                    println!("✅ Fasting goal updated successfully.");
                }
            }
            Some(7) => {
                if let Err(e) = remove_fasting_goal(conn, user.id) {
                    eprintln!("❌ Error removing fasting goal: {}", e);
                } else {
                    println!("✅ Fasting goal removed successfully.");
                }
            }
            Some(8) => break,
            _ => println!("❌ Invalid choice. Please select a valid option."),
        }
    }
}

/// ✅ Handles the analytics-related menu actions.
fn handle_analytics_menu(conn: &mut SqliteConnection, user: &User) {
    loop {
        println!("\nAnalytics Menu:");
        println!("1. Fasting History");
        println!("2. Average Fasting Duration");
        println!("3. Total Fasting Time");
        println!("4. View All Fasting Sessions");
        println!("5. Back to Main Menu");

        match prompt_user_choice("Enter your choice (1-5): ") {
            Some(1) => show_fasting_history(conn, user.id),
            Some(2) => match calculate_average_fasting_duration(conn, user.id) {
                Ok(Some(avg)) => println!("📊 Average Fasting Duration: {} minutes.", avg),
                Ok(None) => println!("❌ No fasting data available."),
                Err(e) => eprintln!("❌ Error calculating average fasting duration: {}", e),
            },
            Some(3) => match calculate_total_fasting_time(conn, user.id) {
                Ok(total) => println!("📊 Total Fasting Time: {} minutes.", total),
                Err(e) => eprintln!("❌ Error calculating total fasting time: {}", e),
            },
            Some(4) => match get_user_fasting_sessions(conn, user.id) {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        println!("❌ No fasting sessions found for user {}.", user.id);
                    } else {
                        println!("📜 Fasting sessions for user {}:", user.id);
                        for session in sessions {
                            println!(
                                "- Start: {}, End: {:?}",
                                session.start_time,
                                session.stop_time.unwrap_or_else(|| Utc::now().naive_utc())
                            );
                        }
                    }
                }
                Err(e) => eprintln!("❌ Error retrieving fasting sessions: {}", e),
            },
            Some(5) => break,
            _ => println!("❌ Invalid choice. Please select a valid option."),
        }
    }
}
