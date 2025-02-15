pub mod analytics;
pub mod fasting;
pub mod goals;
pub mod menu;

// Ensure all necessary functions are accessible from handlers
pub use analytics::{
    calculate_average_fasting_duration, 
    calculate_total_fasting_time, 
    show_fasting_history,
};

pub use fasting::{
    get_current_fasting_status, 
    start_fasting, 
    stop_fasting
};

pub use goals::{
    add_goal,
    view_goals,
};

pub use menu::display_main_menu;