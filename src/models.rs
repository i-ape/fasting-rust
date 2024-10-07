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
pub struct NewUser {
    pub username: String,
    pub password: String,
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
    pub end_time: Option<NaiveDateTime>,
}
