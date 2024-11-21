pub mod analytics;
pub mod fasting;
pub mod user;

pub use analytics::{calculate_average_fasting_duration}; 
pub use fasting::{start_fasting, stop_fasting};
pub use user::{create_user, find_user_by_username, login_user, register_user, update_user_profile};
