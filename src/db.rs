use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

use crate::errors::FastingAppError;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

/// Establish a direct connection to SQLite
pub fn establish_connection() -> Result<SqliteConnection, FastingAppError> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        FastingAppError::InvalidRequest("DATABASE_URL must be set".to_string())
    })?;
    
    SqliteConnection::establish(&database_url)
        .map_err(|_| FastingAppError::ConnectionError) // Map Diesel errors to your ConnectionError
}

/// Create a connection pool for multithreaded applications
pub fn establish_pool() -> Result<DbPool, FastingAppError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        FastingAppError::InvalidRequest("DATABASE_URL must be set".to_string())
    })?;
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .map_err(|_| FastingAppError::ConnectionError)
}

/// Helper function to get a connection from the pool
pub fn get_connection(pool: &DbPool) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>, FastingAppError> {
    pool.get()
        .map_err(|_| FastingAppError::ConnectionError) // Map connection pool errors to your ConnectionError
}
