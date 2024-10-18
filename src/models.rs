use diesel::{Queryable, Insertable};
use serde::Serialize;

use crate::schema::{users, fasting_events};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_password: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct FastingEvent {
    pub id: i32,
    pub user_id: i32,
    pub start_time: chrono::NaiveDateTime,
    pub stop_time: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "fasting_events"]
pub struct NewFastingEvent {
    pub user_id: i32,
    pub start_time: chrono::NaiveDateTime,
    pub stop_time: Option<chrono::NaiveDateTime>,
}
