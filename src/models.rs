use crate::schema::{fasting_events, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
//use diesel::sqlite::SqliteConnection;
use diesel::{Identifiable, Insertable, Queryable};

/// Represents a user in the `users` table.
#[derive(Debug, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
}

/// Used to insert a new user into the `users` table.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

/// Represents a fasting event in the `fasting_events` table.
#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "fasting_events"]
pub struct FastingEvent {
    pub id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // `None` means the fast is still ongoing
}

/// Used to insert a new fasting event into the `fasting_events` table.
#[derive(Insertable)]
#[table_name = "fasting_events"]
pub struct NewFastingEvent {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // `None` when the fast starts
}
