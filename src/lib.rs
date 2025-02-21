pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod users;

pub use handlers::{
    fasting::{start_fasting, stop_fasting, get_current_fasting_status, get_user_fasting_sessions}, 
    analytics::{calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history},
    goals::{add_goal, view_goals},
};

pub use users::{
    update::update_user_profile,
    create::{create_user, register_user},
    login::{login_user, find_user_by_device_id, associate_device_id, login_user_or_device}

}; // From `users/mod.rs`

pub use crate::errors::FastingAppError::*;
