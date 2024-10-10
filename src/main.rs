extern crate bcrypt;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod models;
mod schema;

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

    // Retrieve the database URL and establish the connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database URL: {}", database_url);

    // Establish the connection to the database as mutable
    let mut connection = establish_connection();

    // Parse command-line arguments
    let args = Cli::from_args();

    // Match and execute the relevant subcommand
    match args.command {
        Command::Register { username, password } => {
            println!("Registering user: {} with password: {}", username, password);
            // Call the create_user function with a mutable reference to the connection
            create_user(&mut connection, &username, &password).expect("Error creating user");
        }
        Command::Login { username, password } => {
            println!(
                "Attempting to log in user: {} with password: {}",
                username, password
            );
            // Implement login logic here
        }
        Command::StartFasting { user_id } => {
            println!("Starting fasting session for user ID: {}", user_id);
            start_fasting(&mut connection, user_id).expect("Error starting fasting session");
        }
        Command::StopFasting { session_id } => {
            println!("Stopping fasting session with ID: {}", session_id);
            stop_fasting(&mut connection, session_id).expect("Error stopping fasting session");
        }
    }
}
