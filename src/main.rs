extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
//use log;

use crate::db::establish_connection;
use handlers::menu::display_main_menu;
mod db;
mod errors;
mod models;
mod schema;
pub mod handlers {
    pub mod analytics;
    pub mod fasting;
    pub mod goals;
    pub mod menu;
}
pub mod users {
    pub mod create;
    pub mod find;
    pub mod login;
    pub mod update;
}

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
