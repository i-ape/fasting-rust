pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod users;

pub use handlers::menu::display_main_menu;
pub use handlers::fasting::{start_fasting, stop_fasting, get_current_fasting_status};
pub use handlers::analytics::{calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history};
pub use handlers::goals::{add_goal, view_goals};

pub use users::login::{login_user, associate_device_id};
pub use users::find::find_user_by_id;
pub use users::update::update_user_profile;
 // From `users/mod.rs`

pub use crate::errors::FastingAppError::*;
