extern crate bcrypt;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod models;
mod schema; // Include the schema

use crate::db::establish_connection;
use crate::handlers::{create_user, login_user, start_fasting, stop_fasting};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Register { username: String, password: String },
    Login { username: String, password: String },
    StartFasting { user_id: i32 },
    StopFasting { session_id: i32 },
}

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Establish the connection to the database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database URL: {}", database_url);

    // Establish the connection to the database
    let mut connection = establish_connection();

    // Parse command-line arguments
    let args = Cli::from_args();

    // Match and execute the relevant subcommand
    match args.command {
        Command::Register { username, password } => {
            println!("Registering user: {} with password: {}", username, password);
            match create_user(&mut connection, &username, &password) {
                Ok(user) => println!("User registered successfully with ID: {}", user.id),
                Err(e) => eprintln!("Error registering user: {}", e),
            }
        }
        Command::Login { username, password } => {
            println!(
                "Attempting to log in user: {} with password: {}",
                username, password
            );
            match login_user(&mut connection, &username, &password) {
                Ok(is_valid) => {
                    if is_valid {
                        println!("Login successful!");
                    } else {
                        println!("Invalid credentials.");
                    }
                }
                Err(e) => eprintln!("Error logging in: {}", e),
            }
        }
        Command::StartFasting { user_id } => {
            println!("Starting fasting session for user ID: {}", user_id);
            match start_fasting(&mut connection, user_id) {
                Ok(session) => println!("Started fasting session with ID: {}", session.id),
                Err(e) => eprintln!("Error starting fasting session: {}", e),
            }
        }
        Command::StopFasting { session_id } => {
            println!("Stopping fasting session with ID: {}", session_id);
            match stop_fasting(&mut connection, session_id) {
                Ok(session) => println!("Stopped fasting session with ID: {}", session.id),
                Err(e) => eprintln!("Error stopping fasting session: {}", e),
            }
        }
    }
}
