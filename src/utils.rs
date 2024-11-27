use std::io::{self, Write};
use crate::errors::FastingAppError;

pub fn prompt_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn handle_error(error: FastingAppError) {
    match error {
        FastingAppError::InvalidRequest(msg) => eprintln!("Invalid request: {}", msg),
        FastingAppError::DatabaseError(e) => eprintln!("Database error: {}", e),
        FastingAppError::PasswordHashError(e) => eprintln!("Password hash error: {}", e),
        FastingAppError::ExistingSessionError => eprintln!("An existing fasting session is already active."),
        FastingAppError::InvalidCredentials => eprintln!("Invalid username or password."),
        FastingAppError::ConnectionError(err) => eprintln!("Connection error: {}", err),
    }
}
