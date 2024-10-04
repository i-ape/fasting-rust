//extern crate dotenv;
//use diesel::connection;
use dotenv::dotenv;
use std::env;

mod db; // Declares the `db` module (from src/db.rs)
mod handlers; // Declares the `handlers` module (from src/handlers.rs)

use crate::db::establish_connection;
use crate::handlers::{create_user, start_fasting, stop_fasting};
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

    // Establish the connection to the database
    let _connection = establish_connection();

    // Parse command-line arguments
    let args = Cli::from_args();

    // Match and execute the relevant subcommand
    match args.command {
        Command::Register { username, password } => {
            println!("Registering user: {} with password: {}", username, password);
            create_user(&_connection, &username, &password); // Ensure the correct function signature
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
            start_fasting(user_id); // Pass user_id directly
        }
        Command::StopFasting { session_id } => {
            println!("Stopping fasting session with ID: {}", session_id);
            stop_fasting(session_id); // Ensure the correct function signature
        }
    }
}
