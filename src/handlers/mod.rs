pub mod analytics;
pub mod fasting;
pub mod goals;
pub mod menu;

use crate::handlers::analytics::{
    calculate_average_fasting_duration,
    calculate_total_fasting_time,
    get_fasting_sessions,
    show_fasting_history,
};

pub use fasting::{get_current_fasting_status, start_fasting, stop_fasting};
