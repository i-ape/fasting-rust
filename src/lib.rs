pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod users;

pub use handlers::{
    fasting::{start_fasting, stop_fasting, get_current_fasting_status}, 
    analytics::{calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history},
    goals::{add_goal, view_goals},
};

pub use users::{create_user, login_user, register_user}; // From `users/mod.rs`

pub use errors::FastingAppError;
