use thiserror::Error;
use diesel::result::Error as DieselError;
use bcrypt::BcryptError;

/// Centralized error type for the Fasting App.
#[derive(Debug, Error)]
pub enum FastingAppError {
    /// Represents a Diesel database error.
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),

    /// Represents an error when hashing a password with bcrypt.
    #[error("Password hash error: {0}")]
    PasswordHashError(#[from] BcryptError),

    /// Represents an attempt to start a fasting session when one is already active.
    #[error("An existing fasting session is already active for user {0}.")]
    ExistingSessionError(i32),

    /// Represents a generic invalid request.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Represents an error when establishing a database connection.
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// Represents a generic custom error.
    #[error("Custom error: {0}")]
    Custom(String),

    /// Represents an error when login credentials are invalid.
    #[error("Invalid username or password for user: {0}.")]
    InvalidCredentials(String),  // ✅ Stores username

    /// Represents an error related to session handling.
    #[error("Session error: {0}")]
    SessionError(String),
}
