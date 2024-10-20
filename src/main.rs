extern crate bcrypt;
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate structopt;

use chrono::Utc;
use dotenv::dotenv;
use structopt::StructOpt;

mod db;
mod handlers;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::handlers::{create_user, login_user, start_fasting, stop_fasting};

/// Define the command-line options
#[derive(StructOpt, Debug)]
#[structopt(name = "FastingApp", about = "A simple fasting application.")]
struct Cli {
    /// The action to perform (create_user, login, start_fast, stop_fast)
    #[structopt(short, long)]
    action: String,

    /// Username for the operation
    #[structopt(short, long)]
    username: Option<String>,

    /// Password for the operation (required for create_user and login)
    #[structopt(short, long)]
    password: Option<String>,

    /// Event ID for fasting operations (required for start_fast and stop_fast)
    #[structopt(short, long)]
    event_id: Option<i32>,
}

fn main() {
    dotenv().ok();

    // Establish the database connection
    let conn = establish_connection();

    // Parse the command-line arguments
    let args = Cli::from_args();

    // Ensure the action is provided
    match args.action.as_str() {
        "create_user" => {
            // Validate that username and password are provided
            if let (Some(username), Some(password)) = (args.username, args.password) {
                match create_user(&conn, &username, &password) {
                    Ok(_) => println!("User created successfully"),
                    Err(e) => println!("Error creating user: {:?}", e),
                }
            } else {
                println!("Error: Both username and password must be provided for creating a user.");
            }
        }
        "login" => {
            // Validate that username and password are provided
            if let (Some(username), Some(password)) = (args.username, args.password) {
                match login_user(&conn, &username, &password) {
                    Ok(_) => println!("Login successful"),
                    Err(e) => println!("Error logging in: {:?}", e),
                }
            } else {
                println!("Error: Both username and password must be provided for logging in.");
            }
        }
        "start_fast" => {
            // Validate that event ID is provided
            if let Some(event_id) = args.event_id {
                match start_fasting(&conn, event_id, Utc::now().naive_utc()) {
                    Ok(_) => println!("Fasting session started"),
                    Err(e) => println!("Error starting fasting session: {:?}", e),
                }
            } else {
                println!("Error: Event ID must be provided to start a fasting session.");
            }
        }
        "stop_fast" => {
            // Validate that event ID is provided
            if let Some(event_id) = args.event_id {
                match stop_fasting(&conn, event_id, Utc::now().naive_utc()) {
                    Ok(_) => println!("Fasting session stopped"),
                    Err(e) => println!("Error stopping fasting session: {:?}", e),
                }
            } else {
                println!("Error: Event ID must be provided to stop a fasting session.");
            }
        }
        _ => {
            println!("Invalid action. Use 'create_user', 'login', 'start_fast', or 'stop_fast'.");
        }
    }
}
