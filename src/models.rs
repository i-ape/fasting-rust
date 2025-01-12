use crate::schema::{fasting_events, fasting_goals, users};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

/// Represents a user in the database.
#[derive(Queryable, Insertable, AsChangeset, Identifiable, Selectable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,                // Nullable<Integer>
    pub username: String,              // Text
    pub hashed_password: String,       // Text
    pub device_id: Option<String>,     // Nullable<Text>
    pub created_at: Option<chrono::NaiveDateTime>, // Nullable<Timestamp>
}

/// Represents a new user to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

/// Represents a fasting event in the database.
#[derive(Queryable, Identifiable, Debug, Selectable)]
#[diesel(table_name = fasting_events)]
pub struct FastingEvent {
    pub id: Option<i32>,              // Nullable<Integer>
    pub user_id: i32,                 // Integer
    pub start_time: NaiveDateTime,    // Timestamp
    pub stop_time: Option<NaiveDateTime>, // Nullable<Timestamp>
    pub created_at: Option<NaiveDateTime>, // Nullable<Timestamp>
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
#[derive(Queryable, Identifiable, Debug, Selectable)]
#[diesel(table_name = fasting_sessions)]
pub struct FastingSession {
    pub id: Option<i32>,               // Nullable<Integer>
    pub user_id: i32,                  // Integer
    pub start_time: NaiveDateTime,     // Timestamp
    pub stop_time: Option<NaiveDateTime>, // Nullable<Timestamp>
    pub created_at: Option<NaiveDateTime>, // Nullable<Timestamp>
}