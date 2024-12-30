use crate::schema::{fasting_events, users};
use chrono::NaiveDateTime;
use diesel::prelude::*; // Diesel prelude includes commonly used traits.

/// Represents a user in the database.
#[derive(Queryable, Insertable, AsChangeset, Identifiable, Selectable, Debug)] // Added Selectable
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,                // Nullable<Integer>
    pub username: String,              // Text
    pub hashed_password: String,       // Text
    pub device_id: Option<String>,     // Nullable<Text>
    pub created_at: Option<NaiveDateTime>, // Nullable<Timestamp>
}

/// Represents a new user to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,             // Username for the new user.
    pub hashed_password: String,      // Hashed password for the new user.
}

/// Represents a fasting event in the database.
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = fasting_events)]
pub struct FastingEvent {
    pub id: Option<i32>,              // Fasting event ID (nullable for new inserts).
    pub user_id: i32,                 // ID of the user who started the event.
    pub start_time: NaiveDateTime,    // Start time of the fasting event.
    pub stop_time: Option<NaiveDateTime>, // Optional stop time (null if ongoing).
    pub created_at: Option<NaiveDateTime>, // Timestamp of fasting event creation.
}

/// Represents a new fasting event to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = fasting_events)]
pub struct NewFastingEvent {
    pub user_id: i32,                 // ID of the user who is starting the event.
    pub start_time: NaiveDateTime,    // Start time of the fasting event.
    pub stop_time: Option<NaiveDateTime>, // Optional stop time (null for ongoing).
}
#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct FastingSession {
    pub id: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Optional, since a session might still be ongoing
}