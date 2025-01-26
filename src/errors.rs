use bcrypt::BcryptError;
use diesel::result::Error as DieselError;

/// Centralized error type for the Fasting App.
#[derive(Debug)]
pub enum FastingAppError {
    DatabaseError(DieselError),
    PasswordHashError(BcryptError),
    ExistingSessionError,
    InvalidRequest(String),
    ConnectionError(String),
    Custom(String),
    InvalidCredentials, 
}

/// Implement `std::fmt::Display` for user-friendly error messages.
impl std::fmt::Display for FastingAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastingAppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            FastingAppError::PasswordHashError(e) => write!(f, "Password hash error: {}", e),
            FastingAppError::ExistingSessionError => write!(f, "An existing fasting session is already active."),
            FastingAppError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            FastingAppError::ConnectionError(err) => write!(f, "Connection error: {}", err),
            FastingAppError::Custom(msg) => write!(f, "Custom error: {}", msg),
            FastingAppError::InvalidCredentials => write!(f, "Invalid username or password."),
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
