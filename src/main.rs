extern crate diesel;
extern crate dotenv;

mod db;
mod handlers;
mod models;
mod schema;

use crate::db::establish_connection;
use crate::handlers::{create_user, get_session_history, login_user, start_fasting, stop_fasting};
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
    SessionHistory { user_id: i32 },
}

fn main() {
    dotenv::dotenv().ok();

    // Establish the database connection
    let mut connection = establish_connection();
    let args = Cli::from_args();

    match args.command {
        Command::Register { username, password } => {
            match create_user(&mut connection, &username, &password) {
                Ok(user) => println!("Successfully registered user: {}", user.username),
                Err(err) => println!("Error creating user: {:?}", err),
            }
        }
        Command::Login { username, password } => {
            match login_user(&mut connection, &username, &password) {
                Ok(user) => println!("User {} logged in successfully", user.username),
                Err(err) => println!("Login failed: {:?}", err),
            }
        }
        Command::StartFasting { user_id } => match start_fasting(&mut connection, user_id) {
            Ok(session) => println!("Started fasting session with ID: {}", session.id),
            Err(err) => println!("Error starting session: {:?}", err),
        },
        Command::StopFasting { session_id } => match stop_fasting(&mut connection, session_id) {
            Ok(session) => println!("Stopped fasting session with ID: {}", session.id),
            Err(err) => println!("Error stopping session: {:?}", err),
        },
        Command::SessionHistory { user_id } => {
            match get_session_history(&mut connection, user_id) {
                Ok(sessions) => {
                    println!("Fasting history for user ID: {}", user_id);
                    for session in sessions {
                        let start = session.start_time;
                        let end = session.end_time.unwrap_or(Utc::now().naive_utc());
                        let duration = handlers::calculate_session_duration(start, end);
                        println!(
                            "Session ID: {}, Start: {}, End: {}, Duration: {}",
                            session.id, start, end, duration
                        );
                    }
                }
                Err(err) => println!("Error retrieving session history: {:?}", err),
            }
        }
    }
}
