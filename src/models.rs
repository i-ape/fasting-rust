use crate::schema::{fasting_sessions, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use diesel::Queryable;

// User table model
#[derive(Queryable)]
pub struct User {
    pub id: Option<i32>, // Since the ID is nullable in the schema, make it `Option<i32>`
    pub username: String,
    pub hashed_password: String, // Match the field name in your schema
    pub created_at: Option<NaiveDateTime>, // Match nullable `created_at`
}

// Structure for inserting new users
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

// FastingSession table model
#[derive(Queryable)]
pub struct FastingSession {
    pub id: Option<i32>, // Match the nullable ID from the schema
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Nullable in the schema
}

// Structure for inserting new fasting sessions
#[derive(Insertable)]
#[table_name = "fasting_sessions"]
pub struct NewFastingSession {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Nullable
}
