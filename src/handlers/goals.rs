use crate::errors::{handle_error, FastingAppError};
use crate::models::{FastingGoal, NewFastingGoal};
use crate::schema::fasting_goals::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::io::{self, Write};

/// Adds a new fasting goal for the user.
pub fn add_goal(user_id: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    println!("Enter the goal duration in hours:");

    // Read and validate the duration input
    let duration_hours: i32 = match prompt_input::<i32>("Goal Duration (hours): ") {
        Some(hours) if hours > 0 => hours,
        _ => {
            println!("Invalid duration. Please enter a positive number.");
            return Err(FastingAppError::InvalidRequest(
                "Invalid duration entered.".to_string(),
            ));
        }
    };

    // Read and validate the deadline input
    let deadline: NaiveDateTime = match prompt_datetime("Deadline (YYYY-MM-DD HH:MM): ") {
        Some(date) => date,
        None => {
            println!("Invalid deadline. Please use the format YYYY-MM-DD HH:MM.");
            return Err(FastingAppError::InvalidRequest(
                "Invalid deadline entered.".to_string(),
            ));
        }
    };

    // Create a new fasting goal
    let new_goal = NewFastingGoal {
        user_id,
        goal_duration: duration_hours,
        deadline,
        created_at: Some(chrono::Utc::now().naive_utc()),
    };

    // Insert the new goal into the database
    diesel::insert_into(fasting_goals)
        .values(&new_goal)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)?;

    println!("Goal added successfully!");
    Ok(())
}

/// Displays the fasting goals for the given user.
pub fn view_goals(user_id_param: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    // Retrieve fasting goals for the given user
    let goals = fasting_goals
        .filter(user_id.eq(user_id_param))
        .load::<FastingGoal>(conn)
        .map_err(FastingAppError::DatabaseError)?;

    if goals.is_empty() {
        println!("No fasting goals found for user ID {}.", user_id_param);
    } else {
        println!("Fasting goals for user ID {}:", user_id_param);
        for goal in goals {
            println!(
                "Goal: {} hours, Deadline: {}, Created At: {}",
                goal.goal_duration,
                goal.deadline,
                goal.created_at.unwrap_or_else(|| "N/A".to_string())
            );
        }
    }
    Ok(())
}

/// Prompts the user for input and parses it into the desired type.
fn prompt_input<T: std::str::FromStr>(message: &str) -> Option<T> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok()
}

/// Prompts the user for a valid `NaiveDateTime` input.
fn prompt_datetime(message: &str) -> Option<NaiveDateTime> {
    loop {
        if let Some(input) = prompt_input::<String>(message) {
            if let Ok(date) = NaiveDateTime::parse_from_str(&input, "%Y-%m-%d %H:%M") {
                return Some(date);
            } else {
                println!("Invalid date format. Please try again.");
            }
        } else {
            return None;
        }
    }
}
