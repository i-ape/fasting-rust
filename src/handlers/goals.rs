use diesel::prelude::*;
use crate::models::Goal;
use crate::schema::goals::dsl::*;

pub fn add_goal(conn: &mut PgConnection) {
    println!("Enter your goal description:");

    let mut description = String::new();
    std::io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");

    let new_goal = Goal {
        id: uuid::Uuid::new_v4().to_string(),
        description: description.trim().to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    match diesel::insert_into(goals)
        .values(&new_goal)
        .execute(conn)
    {
        Ok(_) => println!("Goal added successfully!"),
        Err(e) => eprintln!("Error adding goal: {:?}", e),
    }
}


pub fn view_goals(user_id: i32, conn: &mut SqliteConnection) -> Result<(), FastingAppError> {
    // Example logic: Retrieve and display goals from the database
    let goals = fasting_goals::table
        .filter(fasting_goals::user_id.eq(user_id))
        .load::<FastingGoal>(conn)
        .map_err(FastingAppError::DatabaseError)?;

    if goals.is_empty() {
        println!("No fasting goals found.");
    } else {
        for goal in goals {
            println!("Goal: {} hours by {}", goal.duration_hours, goal.deadline);
        }
    }
    Ok(())
}
