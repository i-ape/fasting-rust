use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use thiserror::Error;

/// Centralized error type for the Fasting App.
#[derive(Debug, Error)]
pub enum FastingAppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Password hash error: {0}")]
    PasswordHashError(#[from] BcryptError),
    #[error("An existing fasting session is already active.")]
    ExistingSessionError,
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("No ongoing fasting session found.")]
    SessionError(String),
}
