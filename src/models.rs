use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::users;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct FastingSession {
    pub id: i32,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}
