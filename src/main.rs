extern crate bcrypt;
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate structopt;

use chrono::Utc;
use dotenv::dotenv;
use std::io::{self, Write};  // For user input
use structopt::StructOpt;

mod db;
mod handlers;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::handlers::{create_user, login_user, start_fasting, stop_fasting};

// Helper function to prompt user for input
fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();  // Make sure the prompt shows immediately
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()  // Remove any extra newlines
}

fn main() {
    dotenv().ok();

    // Establish the database connection
    let conn = establish_connection();

    // Get input from the user
    let username = prompt_input("Enter username: ");
    let password = prompt_input("Enter password: ");

    // Example of creating a user
    match create_user(&conn, &username, &password) {
        Ok(_) => println!("User created successfully"),
        Err(e) => println!("Error creating user: {:?}", e),
    }

    // Example of logging in a user
    match login_user(&conn, &username, &password) {
        Ok(_valid) => {
            println!("Login successful");
        }
        Err(e) => println!("Error logging in: {:?}", e),
    }

    // Example of starting a fasting session
    match start_fasting(&conn, 1, Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session started"),
        Err(e) => println!("Error starting fasting session: {:?}", e),
    }

    // Example of stopping a fasting session
    match stop_fasting(&conn, 1, Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session stopped"),
        Err(e) => println!("Error stopping fasting session: {:?}", e),
    }
}
