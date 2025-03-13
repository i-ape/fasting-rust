extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
//use log;

use crate::db::establish_connection;
use handlers::menu::display_main_menu;
pub use users::{create_user, login_user, login, register_user};

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