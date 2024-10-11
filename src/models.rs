use crate::schema::{fasting_sessions, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};

// Users table model
#[derive(Queryable)]
pub struct User {
    pub id: Option<i32>, // The id is nullable in your schema
    pub username: String,
    pub hashed_password: String,
    pub created_at: Option<NaiveDateTime>, // Nullable in schema
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
    pub id: Option<i32>, // The id is nullable in your schema
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Nullable in schema
}

// Structure for inserting new fasting sessions
#[derive(Insertable)]
#[table_name = "fasting_sessions"]
pub struct NewFastingSession {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>, // Nullable
}
