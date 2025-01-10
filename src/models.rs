use crate::schema::{fasting_events, fasting_goals, users};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

/// Represents a user in the database.
#[derive(Queryable, Insertable, AsChangeset, Identifiable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32, // Primary keys are not nullable
    pub username: String,
    pub hashed_password: String,
    pub device_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// Represents a new user to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

/// Represents a fasting event in the database.
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = fasting_events)]
pub struct FastingEvent {
    pub id: i32, // Primary key
    pub user_id: i32, // Foreign key to users table
    pub start_time: NaiveDateTime,
    pub stop_time: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

/// Represents a new fasting event to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = fasting_events)]
pub struct NewFastingEvent {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub stop_time: Option<NaiveDateTime>,
}

/// Represents a fasting goal.
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = fasting_goals)]
pub struct FastingGoal {
    pub id: i32,
    pub user_id: i32,
    pub goal_duration: i32,
    pub deadline: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
}

/// Represents a fasting session.
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = fasting_sessions)]
pub struct FastingSession {
    pub id: String,                   // TEXT
    pub start_time: NaiveDateTime,    // TIMESTAMP
    pub end_time: Option<NaiveDateTime>, // Nullable<TIMESTAMP>
}