use crate::schema::{fasting_events, users};
use chrono::NaiveDateTime;
use diesel::{prelude::Identifiable, Insertable, Queryable};

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

#[derive(Queryable, Identifiable)]
#[table_name = "fasting_events"]
pub struct FastingEvent {
    pub id: Option<i32>,
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub stop_time: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "fasting_events"]
pub struct NewFastingEvent {
    pub user_id: i32,
    pub start_time: NaiveDateTime,
    pub stop_time: Option<NaiveDateTime>,
}
