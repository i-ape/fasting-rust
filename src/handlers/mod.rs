// src/handlers/mod.rs

pub mod fasting;
pub mod analytics;

pub use fasting::{
    get_fasting_checkpoints,
    get_fasting_history,
    calculate_average_fasting_duration,
    calculate_weekly_fasting_summary,
    calculate_current_streak,
    calculate_total_fasting_time,
};

pub use analytics::{
    calculate_user_performance,
    generate_fasting_report,
    get_fasting_trends,
    generate_statistics,
};