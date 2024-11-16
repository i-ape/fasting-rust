use bcrypt::BcryptError;
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum FastingAppError {
    DatabaseError(DieselError),
    PasswordHashError(BcryptError),
    ExistingSessionError,
    InvalidCredentials,
    InvalidRequest(String),
}

impl From<DieselError> for FastingAppError {
    fn from(error: DieselError) -> Self {
        FastingAppError::DatabaseError(error)
    }
}

impl From<BcryptError> for FastingAppError {
    fn from(error: BcryptError) -> Self {
        FastingAppError::PasswordHashError(error)
    }
}
