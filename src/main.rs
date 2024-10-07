extern crate diesel;
extern crate dotenv;

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
    dotenv::dotenv().ok();

    let connection = establish_connection();
    let args = Cli::from_args();

    match args.command {
        Command::Register { username, password } => {
            match create_user(&connection, &username, &password) {
                Ok(user) => println!("Successfully registered user: {}", user.username),
                Err(err) => println!("Error creating user: {:?}", err),
            }
        }
        Command::Login { username, password } => {
            match login_user(&connection, &username, &password) {
                Ok(user) => println!("User {} logged in successfully", user.username),
                Err(err) => println!("Login failed: {:?}", err),
            }
        }
        Command::StartFasting { user_id } => {
            match start_fasting(&connection, user_id) {
                Ok(session) => println!("Started fasting session with ID: {}", session.id),
                Err(err) => println!("Error starting session: {:?}", err),
            }
        }
        Command::StopFasting { session_id } => {
            match stop_fasting(&connection, session_id) {
                Ok(session) => println!("Stopped fasting session with ID: {}", session.id),
                Err(err) => println!("Error stopping session: {:?}", err),
            }
        }
    }
}
