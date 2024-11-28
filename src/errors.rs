use bcrypt::BcryptError;
use diesel::result::Error as DieselError;

/// Centralized error type for the Fasting App.
#[derive(Debug)]
pub enum FastingAppError {
    DatabaseError(DieselError),
    PasswordHashError(BcryptError),
    ExistingSessionError,
    InvalidCredentials,
    InvalidRequest(String),
    ConnectionError(String),
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
            eprintln!("Connection error: {}", err);
        }
    }
}

/// Implement `std::fmt::Display` for user-friendly error messages.
impl std::fmt::Display for FastingAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastingAppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            FastingAppError::PasswordHashError(e) => write!(f, "Password hash error: {}", e),
            FastingAppError::ExistingSessionError => write!(f, "An existing fasting session is already active."),
            FastingAppError::InvalidCredentials => write!(f, "Invalid username or password."),
            FastingAppError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            FastingAppError::ConnectionError(err) => write!(f, "Connection error: {}", err),
            #[allow(unreachable_patterns)]
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
#[cfg(test)]
mod tests {
    use super::FastingAppError;

    #[test]
    fn test_connection_error_display() {
        let connection_error = FastingAppError::ConnectionError("Failed to connect to the database".to_string());
        assert_eq!(
            format!("{}", connection_error),
            "Connection error: Failed to connect to the database"
        );
    }
}