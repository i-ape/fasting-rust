use crate::errors::FastingAppError;
use crate::models::FastingGoal;
use crate::schema::fasting_goals::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::io::{self, Write};

/// Adds a new fasting goal for the user.
pub fn add_goal(user_id_param: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    let duration_input = prompt_user_input("Enter the goal duration in hours: ")?;
    let parsed_duration: i32 = duration_input.parse().map_err(|_| {
        FastingAppError::InvalidRequest("Invalid duration entered.".to_string())
    })?;

    let deadline_input = prompt_user_input("Enter the deadline (YYYY-MM-DD HH:MM): ")?;
    let goal_deadline = NaiveDateTime::parse_from_str(&deadline_input, "%Y-%m-%d %H:%M")
        .map_err(|_| FastingAppError::InvalidRequest("Invalid deadline format.".to_string()))?;

    let new_goal = FastingGoal {
        id: None,
        user_id: user_id_param,
        goal_duration: parsed_duration,
        deadline: goal_deadline,
        created_at: Some(chrono::Utc::now().naive_utc()),
    };

    diesel::insert_into(fasting_goals)
        .values(&new_goal)
        .execute(conn)
        .map_err(FastingAppError::DatabaseError)?;

    println!("Goal added successfully!");
    Ok(())
}

/// Displays the fasting goals for the given user.
pub fn view_goals(user_id_param: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    let user_goals = fasting_goals
        .filter(user_id.eq(user_id_param))
        .select(FastingGoal::as_select())
        .load::<FastingGoal>(conn)
        .map_err(FastingAppError::DatabaseError)?;

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
                    .unwrap_or_else(|| "Unknown".to_string())
            );
        }
    }
    Ok(())
}
