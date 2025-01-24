use crate::errors::{handle_error, FastingAppError};
use crate::models::FastingGoal;
use crate::schema::fasting_goals::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;

/// Adds a new fasting goal for the user.
pub fn add_goal(user_id_param: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    // Prompt for goal duration
    println!("Enter the goal duration in hours:");
    let mut duration_input = String::new();
    std::io::stdin()
        .read_line(&mut duration_input)
        .expect("Failed to read input");

    let parsed_duration: i32 = match duration_input.trim().parse() {
        Ok(hours) if hours > 0 => hours,
        _ => {
            println!("Invalid duration. Please enter a positive number.");
            return Err(FastingAppError::InvalidRequest(
                "Invalid duration entered.".to_string(),
            ));
        }
    };

    // Prompt for deadline
    println!("Enter the deadline for this goal (YYYY-MM-DD HH:MM):");
    let mut deadline_input = String::new();
    std::io::stdin()
        .read_line(&mut deadline_input)
        .expect("Failed to read input");

    let goal_deadline: NaiveDateTime =
        match NaiveDateTime::parse_from_str(deadline_input.trim(), "%Y-%m-%d %H:%M") {
            Ok(date) => date,
            Err(_) => {
                println!("Invalid date format. Please use YYYY-MM-DD HH:MM.");
                return Err(FastingAppError::InvalidRequest(
                    "Invalid deadline entered.".to_string(),
                ));
            }
        };

    // Create the new goal
    let new_goal = FastingGoal {
        id: 0, // Placeholder; SQLite will auto-generate this if `id` is nullable
        user_id: user_id_param,
        goal_duration: parsed_duration, // Use renamed variable here
        deadline: goal_deadline,
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
    // Query for the user's fasting goals
    let user_goals = fasting_goals
        .filter(user_id.eq(user_id_param)) // Filter by user ID
        .load::<FastingGoal>(conn)
        .map_err(FastingAppError::DatabaseError)?;

    // Display the goals
    if user_goals.is_empty() {
        println!("No fasting goals found for user ID {}.", user_id_param);
    } else {
        println!("Fasting goals for user ID {}:", user_id_param);
        for goal in user_goals {
            println!(
                "Goal: {} hours, Deadline: {}, Created At: {}",
                goal.goal_duration,
                goal.deadline,
                goal.created_at
                    .map(|ts| ts.to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
            );
        }
    }
    Ok(())
}
