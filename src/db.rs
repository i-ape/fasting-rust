use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

use crate::errors::FastingAppError;

/// Establish a direct connection to SQLite
pub fn establish_connection() -> Result<SqliteConnection, FastingAppError> {
    dotenv().ok(); // Load environment variables from the `.env` file
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        FastingAppError::InvalidRequest("DATABASE_URL must be set".to_string())
    })?;
    
    SqliteConnection::establish(&database_url)
        .map_err(|err| FastingAppError::ConnectionError(format!("Failed to connect: {}", err)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_establish_connection_success() {
        dotenv().ok();
        let connection = establish_connection();
        assert!(connection.is_ok(), "Expected connection to succeed, but it failed.");
    }
}
