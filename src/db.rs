use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use std::env;
use dotenv::dotenv;

type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// Establishes a connection to the SQLite database
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Pool-based connection for multithreaded applications
pub fn establish_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}
