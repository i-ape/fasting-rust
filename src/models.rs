use crate::schema::{fasting_sessions, users};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

// User table model
#[derive(Queryable)]
pub struct User {
    pub id: Option<i32>, // Nullable<Integer> corresponds to Option<i32>
    pub username: String,
    pub hashed_password: String,
    pub created_at: Option<NaiveDateTime>,
}

// Structure for inserting new users
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String, // This should match the `hashed_password` column in `schema.rs`
}

// FastingSession table model
#[derive(Queryable)]
pub struct FastingSession {
    pub id: Option<i32>, // Nullable<Integer> corresponds to Option<i32>
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

// Structure for inserting new fasting sessions
#[derive(Insertable)]
#[table_name = "fasting_sessions"]
pub struct NewFastingSession {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}
