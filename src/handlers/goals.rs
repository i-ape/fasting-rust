use diesel::prelude::*;
use crate::errors::{handle_error, FastingAppError};
use crate::models::FastingGoal;
use crate::schema::fasting_goals::dsl::*;
use chrono::NaiveDateTime;

/// Adds a new fasting goal for the user.
pub fn add_goal(user_id: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    println!("Enter the goal duration in hours:");
    let mut duration_input = String::new();
    std::io::stdin()
        .read_line(&mut duration_input)
        .expect("Failed to read input");

    let duration_hours: i32 = match duration_input.trim().parse() {
        Ok(hours) if hours > 0 => hours,
        _ => {
            println!("Invalid duration. Please enter a positive number.");
            return Err(FastingAppError::InvalidRequest(
                "Invalid duration entered.".to_string(),
            ));
        }
    };

    println!("Enter the deadline for this goal (YYYY-MM-DD HH:MM):");
    let mut deadline_input = String::new();
    std::io::stdin()
        .read_line(&mut deadline_input)
        .expect("Failed to read input");

    let deadline: NaiveDateTime = match NaiveDateTime::parse_from_str(deadline_input.trim(), "%Y-%m-%d %H:%M") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid date format. Please use YYYY-MM-DD HH:MM.");
            return Err(FastingAppError::InvalidRequest(
                "Invalid deadline entered.".to_string(),
            ));
        }
    };

    let new_goal = FastingGoal {
        id: None, // Primary key is auto-generated for SQLite.
        user_id,
        duration_hours,
        deadline,
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(fasting_goals)
        .values(&new_goal)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)?;

    println!("Goal added successfully!");
    Ok(())
}

/// Displays the fasting goals for the given user.
pub fn view_goals(user_id: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    let goals = fasting_goals
        .filter(user_id.eq(user_id)) // Filter by user ID
        .load::<FastingGoal>(conn)
        .map_err(FastingAppError::DatabaseError)?;

    if goals.is_empty() {
        println!("No fasting goals found for user ID {}.", user_id);
    } else {
        println!("Fasting goals for user ID {}:", user_id);
        for goal in goals {
            println!(
                "Goal: {} hours, Deadline: {}, Created At: {}",
                goal.duration_hours, goal.deadline, goal.created_at
            );
        }
    }
    Ok(())
}
