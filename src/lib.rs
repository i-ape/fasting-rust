pub mod db;
pub mod errors;
pub mod models;
pub mod schema;

// ✅ submodules need decaration when mod is not present
pub mod handlers {
    pub mod analytics;
    pub mod fasting;
    pub mod goals;
    pub mod menu;
}

pub mod users {
    pub mod find;
    pub mod login;
    pub mod update;
}

// ✅ Publicly re-export functions so they are accessible from `lib.rs`
pub use handlers::analytics::{
    calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history,
};
pub use handlers::fasting::{get_current_fasting_status, start_fasting, stop_fasting};
pub use handlers::goals::{add_goal, view_goals};
pub use handlers::menu::display_main_menu;

pub use users::find::find_user_by_id;
pub use users::login::{associate_device_id, login_user};
pub use users::update::update_user_profile;

pub use crate::errors::FastingAppError::*;
