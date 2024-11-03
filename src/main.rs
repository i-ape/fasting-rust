extern crate bcrypt;
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate structopt;

use chrono::Utc;
use dotenv::dotenv;
//use std::io::{self, Write};
//use structopt::StructOpt;

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::handlers::{create_user, login_user, start_fasting, stop_fasting};

fn prompt_input(message: &str) -> String {
    use std::io::{self, Write};
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    dotenv().ok();

    // Establish the database connection
    let mut conn = establish_connection();

    // Get input from the user
    let username = prompt_input("Enter username: ");
    let password = prompt_input("Enter password: ");

    // Create a new user
    match create_user(&mut conn, &username, &password) {
        Ok(_) => println!("User created successfully."),
        Err(e) => println!("Error creating user: {:?}", e),
    }

    // Log in the user
    match login_user(&mut conn, &username, &password) {
        Ok(user) => {
            println!("Login successful. User ID: {}", user.id);

            // Start a fasting session
            match start_fasting(&mut conn, user.id, Utc::now().naive_utc()) {
                Ok(_) => println!("Fasting session started."),
                Err(e) => println!("Error starting fasting session: {:?}", e),
            }

            // After some time, stop the fasting session (for demonstration, stopping right after starting)
            match stop_fasting(&mut conn, user.id, Utc::now().naive_utc()) {
                Ok(_) => println!("Fasting session stopped."),
                Err(e) => println!("Error stopping fasting session: {:?}", e),
            }
        }
        Err(e) => println!("Error logging in: {:?}", e),
    }
}
