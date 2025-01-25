use bcrypt::BcryptError;
use diesel::result::Error as DieselError;

/// Centralized error type for the Fasting App.
#[derive(Debug)]
pub enum FastingAppError {
    AuthenticationError(String), // Authentication-related errors
    SessionError(String),        // Fasting session-related errors
    DatabaseError(DieselError),  // Errors from Diesel
    PasswordHashError(BcryptError), // Errors during password hashing
    InvalidRequest(String),      // Errors for invalid user requests
    ConnectionError(String),     // Connection-related errors
}

/// Handles displaying/logging errors in a user-friendly format.
pub fn handle_error(error: FastingAppError) {
    match error {
        FastingAppError::AuthenticationError(msg) => {
            eprintln!("Authentication error: {}", msg);
        }
        FastingAppError::SessionError(msg) => {
            eprintln!("Session error: {}", msg);
        }
        FastingAppError::DatabaseError(e) => {
            eprintln!("Database error: {}", e);
        }
        FastingAppError::PasswordHashError(e) => {
            eprintln!("Password hash error: {}", e);
        }
        FastingAppError::InvalidRequest(msg) => {
            eprintln!("Invalid request: {}", msg);
        }
        FastingAppError::ConnectionError(err) => {
            eprintln!("Connection error: {}", err);
        }
    }
}

/// Implements `std::fmt::Display` for error messages.
impl std::fmt::Display for FastingAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastingAppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            FastingAppError::SessionError(msg) => write!(f, "Session error: {}", msg),
            FastingAppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            FastingAppError::PasswordHashError(e) => write!(f, "Password hash error: {}", e),
            FastingAppError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            FastingAppError::ConnectionError(err) => write!(f, "Connection error: {}", err),
        }
    }
}

/// Implements `std::error::Error` for compatibility with error-handling crates.
impl std::error::Error for FastingAppError {}

/// Automatic conversion from `DieselError` to `FastingAppError`.
impl From<DieselError> for FastingAppError {
    fn from(error: DieselError) -> Self {
        FastingAppError::DatabaseError(error)
    }
}

/// Automatic conversion from `BcryptError` to `FastingAppError`.
impl From<BcryptError> for FastingAppError {
    fn from(error: BcryptError) -> Self {
        FastingAppError::PasswordHashError(error)
    }
}
