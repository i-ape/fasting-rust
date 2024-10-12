use crate::schema::{fasting_sessions, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Nullable, Timestamp};

// User table model
#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub created_at: Option<NaiveDateTime>, // Nullable timestamp field
}

// Structure for inserting new users
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

// FastingSession table model
#[derive(Queryable, Debug)]
pub struct FastingSession {
    pub id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Nullable end_time
}

// Structure for inserting new fasting sessions
#[derive(Insertable)]
#[table_name = "fasting_sessions"]
pub struct NewFastingSession {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Nullable end_time
}
