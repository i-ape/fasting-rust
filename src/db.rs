use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::errors::FastingAppError;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// Establishes a direct connection to SQLite
pub fn establish_connection() -> Result<SqliteConnection, FastingAppError> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        FastingAppError::InvalidRequest("DATABASE_URL must be set".to_string())
    })?;
    SqliteConnection::establish(&database_url).map_err(|err| {
        FastingAppError::DatabaseError(diesel::result::Error::ConnectionError(Box::new(err.to_string())))
    })
}

/// Creates a connection pool for multithreaded applications
pub fn establish_pool() -> Result<DbPool, FastingAppError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        FastingAppError::InvalidRequest("DATABASE_URL must be set".to_string())
    })?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(|err| FastingAppError::DatabaseError(diesel::result::Error::ConnectionError(
            Box::new(err.to_string()),
        )))
}

/// Helper function to get a connection from the pool
pub fn get_connection(pool: &DbPool) -> Result<SqliteConnection, FastingAppError> {
    pool.get()
        .map_err(|e| FastingAppError::DatabaseError(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )))
}
