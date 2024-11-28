extern crate bcrypt;
extern crate diesel;
extern crate dotenv;


use crate::errors::handle_error;
use db::establish_connection;
use dotenv::dotenv;
use users::{login_user, register_user};
mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod users;
mod utils;

fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    dotenv().ok();

    let mut conn = match establish_connection() {
        Ok(connection) => connection,
        Err(e) => {
            handle_error(e);
            return;
        }
    };

    loop {
        let choice = prompt_input("\nChoose an action: register, login, update, or exit: ").to_lowercase();

        match choice.as_str() {
            "register" => handle_register(&mut conn),
            "login" => handle_login(&mut conn),
            "update" => handle_update(&mut conn),
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid action. Please try again."),
        }
    }
}

fn handle_register(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input("Enter a new username: ");
    let password = prompt_input("Enter a new password: ");

    match register_user(conn, &username, &password) {
        Ok(_) => println!("User registered successfully."),
        Err(e) => handle_error(e),
    }
}

fn handle_login(conn: &mut diesel::SqliteConnection) {
    let username = prompt_input("Enter your username: ");
    let password = prompt_input("Enter your password: ");

    match login_user(conn, &username, &password) {
        Ok(user) => {
            if let Some(id) = user.id {
                println!("Login successful. User ID: {}", id);
                manage_fasting_session(conn, id);
            } else {
                println!("Login successful, but User ID is not available.");
            }
        }
        Err(e) => handle_error(e),
    }
}

fn handle_update(conn: &mut diesel::SqliteConnection) {
    let user_id: i32 = match prompt_input("Enter your User ID: ").parse() {
        Ok(id) if id >= 0 => id,
        _ => {
            println!("Invalid User ID. Please enter a positive integer.");
            return;
        }
    };

    let mut new_username = Some(prompt_input("Enter a new username (or press Enter to skip): "));
    let mut new_password = Some(prompt_input("Enter a new password (or press Enter to skip): "));

    if new_username.as_deref() == Some("") {
        new_username = None;
    }
    if new_password.as_deref() == Some("") {
        new_password = None;
    }

    match update_user_profile(conn, user_id, new_username.as_deref(), new_password.as_deref()) {
        Ok(_) => println!("User profile updated successfully."),
        Err(e) => handle_error(e),
    }
}
