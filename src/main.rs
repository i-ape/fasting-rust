extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
//use log;

use crate::db::establish_connection;
use crate::errors::FastingAppError::*;
pub use handlers::{
    fasting::{start_fasting, stop_fasting, get_current_fasting_status}, 
    analytics::{calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history},
    goals::{add_goal, view_goals},
    menu::display_main_menu,
};
pub use users::{create_user, login_user, register_user};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod users;

fn main() {
    // Load environment variables and initialize logging
    dotenv().ok();
    env_logger::init();

    // Establish a database connection
    let mut conn = match establish_connection() {
        Ok(connection) => connection,
        Err(e) => {
            log::error!("Failed to establish connection: {:?}", e);
            ConnectionError;
            return;
        }
    };

    // Display the main menu
    display_main_menu(&mut conn);
}
