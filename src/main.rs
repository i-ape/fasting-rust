extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::process;
//use log;

use crate::db::establish_connection;
use handlers::menu::display_main_menu;
use handlers::fasting::{start_fasting, stop_fasting, get_current_fasting_status};
use handlers::analytics::{calculate_average_fasting_duration, calculate_total_fasting_time, show_fasting_history};
use handlers::goals::{add_goal, view_goals};
pub use users::{create_user, login_user, register_user};

mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod users;

fn main() {
    dotenv().ok();
    env_logger::init();

    let mut conn = match establish_connection() {
        Ok(connection) => connection,
        Err(e) => {
            log::error!("Failed to establish connection: {:?}", e);
            return;
        }
    };

    display_main_menu(&mut conn);
}