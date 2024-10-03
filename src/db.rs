use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "fasting_app.db"; 
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
