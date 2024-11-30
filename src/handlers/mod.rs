pub mod analytics;
pub mod fasting;

// Re-export functions for easier access
pub use fasting::{start_fasting, stop_fasting}; // Ensure this re-exports the functions
