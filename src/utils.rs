use std::io::{self, Write};
use chrono::Utc;
use diesel::SqliteConnection;
use crate::handlers::{start_fasting, stop_fasting};
use crate::errors::FastingAppError;

pub fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn manage_fasting_session(conn: &mut SqliteConnection, user_id: i32) {
    match start_fasting(conn, user_id, Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session started."),
        Err(e) => handle_error(e),
    }

    match stop_fasting(conn, user_id, Utc::now().naive_utc()) {
        Ok(_) => println!("Fasting session stopped."),
        Err(e) => handle_error(e),
    }
}

pub fn handle_error(error: FastingAppError) {
    match error {
        FastingAppError::InvalidRequest(msg) => {
            eprintln!("Invalid request: {}", msg);
        }
        FastingAppError::DatabaseError(e) => {
            eprintln!("Database error: {}", e);
        }
        FastingAppError::PasswordHashError(e) => {
            eprintln!("Password hash error: {}", e);
        }
        FastingAppError::ExistingSessionError => {
            eprintln!("An existing fasting session is already active.");
        }
        FastingAppError::InvalidCredentials => {
            eprintln!("Invalid username or password.");
        }
        FastingAppError::ConnectionError(err) => {
            eprintln!("Failed to connect to the database: {}", err);
        }
    }
}
