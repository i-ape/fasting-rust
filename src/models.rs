use diesel::Queryable;
use diesel::Insertable;
use crate::schema::{users, fasting_sessions};
use chrono::NaiveDateTime;

// User table model
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

// Structure for inserting new users
#[derive(Insertable)]
#[table_name = "users"]
#[derive(Queryable)]
pub struct User {
    pub id: Option<i32>,            // Nullable ID field
    pub username: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>, // Assuming `created_at` is a nullable field
}

}

// FastingSession table model
#[derive(Queryable)]
pub struct FastingSession {
    pub id: i32,
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
    pub end_time: Option<NaiveDateTime>, // This is crucial for `Nullable<Timestamp>`
}

