pub mod analytics;
pub mod fasting;
pub mod goals;
pub mod menu;

pub use fasting::{
    get_current_fasting_status, 
    start_fasting, 
    stop_fasting, 
    get_user_fasting_sessions,
};
pub use analytics::{
    calculate_average_fasting_duration, 
    calculate_total_fasting_time, 
    show_fasting_history
};
pub use goals::{add_goal, view_goals};

pub use menu::display_main_menu; 
