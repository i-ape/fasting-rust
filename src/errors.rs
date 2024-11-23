use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use std::fmt;

/// Centralized error type for the Fasting App.
#[derive(Debug)]
pub enum FastingAppError {
    DatabaseError(DieselError),
    PasswordHashError(BcryptError),
    ExistingSessionError,
    InvalidCredentials,
    InvalidRequest(String),
    ConnectionError(String), // Tuple variant for detailed errors
}


/// Implement `std::fmt::Display` for user-friendly error messages.
impl fmt::Display for FastingAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FastingAppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            FastingAppError::PasswordHashError(e) => write!(f, "Password hashing error: {}", e),
            FastingAppError::ExistingSessionError => write!(f, "An existing fasting session is active."),
            FastingAppError::InvalidCredentials => write!(f, "Invalid username or password."),
            FastingAppError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            FastingAppError::ConnectionError => write!(f, "Failed to connect to the database."),
            _ => write!(f, "An unknown error occurred."),
        }
    }
}

/// Implement `std::error::Error` for compatibility with other error handling crates.
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
