extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
//use log;

use crate::db::establish_connection;
use crate::errors::FastingAppError::*;
use crate::handlers::menu::display_main_menu;

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
