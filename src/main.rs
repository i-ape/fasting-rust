extern crate bcrypt;
extern crate diesel;
extern crate dotenv;
extern crate rocket;

use rocket::serde::json::Json;
use rocket_sync_db_pools::database;

use dotenv::dotenv;
//use std::env;

mod db;
mod handlers;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::handlers::{create_user, login_user, start_fasting, stop_fasting};
//use structopt::StructOpt;

fn main() {
    dotenv().ok();

    // Establish the database connection
    let conn = establish_connection();

    // Example of creating a user
    match create_user(&conn, "example_username", "example_password") {
        Ok(_) => println!("User created successfully"),
        Err(e) => println!("Error creating user: {:?}", e),
    }

    // Example of logging in a user
    match login_user(&conn, "example_username", "example_password") {
        Ok(valid) => {
            if valid {
                println!("Login successful");
            } else {
                println!("Invalid credentials");
            }
        }
        Err(e) => println!("Error logging in: {:?}", e),
    }

    // Example of starting a fasting session
    match start_fasting(&conn, 1) {
        Ok(_) => println!("Fasting session started"),
        Err(e) => println!("Error starting fasting session: {:?}", e),
    }

    // Example of stopping a fasting session
    match stop_fasting(&conn, 1) {
        Ok(_) => println!("Fasting session stopped"),
        Err(e) => println!("Error stopping fasting session: {:?}", e),
    }
}
