extern crate bcrypt;
extern crate diesel;
extern crate dotenv;

use std::io::{self, Write};
use chrono::NaiveDateTime;
use dotenv::dotenv;

use crate::db::establish_connection;
use crate::errors::handle_error;
use crate::handlers::menu::{
    handle_register, handle_login, handle_update, handle_fasting_menu, handle_analytics_menu,
};
use log;

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
            handle_error(e);
            return;
        }
    };

    // Main menu loop
    loop {
        match select_menu_option() {
            MenuOption::Register => handle_register(&mut conn),
            MenuOption::Login => handle_login(&mut conn),
            MenuOption::Update => handle_update(&mut conn),
            MenuOption::Fasting => handle_fasting_menu(&mut conn),
            MenuOption::Analytics => handle_analytics_menu(&mut conn),
            MenuOption::Exit => {
                println!("Goodbye!");
                break;
            }
            MenuOption::Invalid => println!("Invalid action. Please try again."),
        }
    }
}

enum MenuOption {
    Register,
    Login,
    Update,
    Fasting,
    Analytics,
    Exit,
    Invalid,
}

fn select_menu_option() -> MenuOption {
    let choice = prompt_input::<String>(
        "\nChoose an action: register, login, update, fasting, analytics, or exit: ",
    )
    .unwrap_or_default()
    .to_lowercase();

    match choice.as_str() {
        "register" => MenuOption::Register,
        "login" => MenuOption::Login,
        "update" => MenuOption::Update,
        "fasting" => MenuOption::Fasting,
        "analytics" => MenuOption::Analytics,
        "exit" => MenuOption::Exit,
        _ => MenuOption::Invalid,
    }
}
