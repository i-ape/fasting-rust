pub mod analytics;
pub mod fasting;

// Re-export functions for easier access
pub use analytics::calculate_average_fasting_duration;
pub use fasting::{start_fasting, stop_fasting};
