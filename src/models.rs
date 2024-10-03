use super::schema::{fasting_sessions, users};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct FastingSession {
    pub id: i32,
    pub user_id: i32,
    pub start_time: chrono::NaiveDateTime,
    pub end_time: Option<chrono::NaiveDateTime>,
}
