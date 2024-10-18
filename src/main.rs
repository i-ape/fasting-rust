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

#[derive(StructOpt)]
struct Cli {
    /// Username for the user
    #[structopt(short, long)]
    username: String,

    /// Password for the user
    #[structopt(short, long)]
    password: String,

    /// Action to perform: create_user, login, start_fast, stop_fast
    #[structopt(short, long)]
    action: String,

    /// Optional fasting event ID for stopping fasts
    #[structopt(short, long, default_value = "1")]
    event_id: i32,
}

fn main() {
    dotenv().ok();

    // Establish the database connection
    let conn = establish_connection();

    // Parse the command-line arguments
    let args = Cli::from_args();

    // Perform action based on the command-line argument
    match args.action.as_str() {
        "create_user" => match create_user(&conn, &args.username, &args.password) {
            Ok(_) => println!("User created successfully"),
            Err(e) => println!("Error creating user: {:?}", e),
        },
        "login" => match login_user(&conn, &args.username, &args.password) {
            Ok(_valid) => {
                println!("Login successful");
            }
            Err(e) => println!("Error logging in: {:?}", e),
        },
        "start_fast" => match start_fasting(&conn, args.event_id, Utc::now().naive_utc()) {
            Ok(_) => println!("Fasting session started"),
            Err(e) => println!("Error starting fasting session: {:?}", e),
        },
        "stop_fast" => match stop_fasting(&conn, args.event_id, Utc::now().naive_utc()) {
            Ok(_) => println!("Fasting session stopped"),
            Err(e) => println!("Error stopping fasting session: {:?}", e),
        },
        _ => {
            println!("Invalid action. Use 'create_user', 'login', 'start_fast', or 'stop_fast'.");
        }
    }
}
