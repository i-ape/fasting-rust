pub mod analytics;
pub mod fasting;
pub mod goals;
pub mod menu;

// Re-export functions for easy access from `handlers` module
pub use analytics::{
    calculate_average_fasting_duration, calculate_current_streak, calculate_total_fasting_time,
    calculate_weekly_fasting_summary, get_fasting_checkpoints, get_fasting_history,
};

pub use fasting::{get_current_fasting_status, start_fasting, stop_fasting};
