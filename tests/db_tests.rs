#[cfg(test)]
mod db_tests {
    use dotenv::dotenv;
    use std::env;
    use crate::fasting-rust::db::{establish_connection, establish_pool, get_connection};
    use crate::errors::FastingAppError;

    #[test]
    fn test_establish_connection_success() {
        dotenv().ok(); // Load .env
        let connection = establish_connection();
        assert!(connection.is_ok(), "Expected connection to succeed, but it failed.");
    }

    #[test]
    fn test_establish_connection_invalid_database_url() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "invalid_path"); // Simulate invalid URL

        let connection = establish_connection();
        assert!(connection.is_err(), "Expected connection to fail, but it succeeded.");

        if let Err(FastingAppError::ConnectionError(err)) = connection {
            assert!(err.contains("Failed to connect"), "Unexpected error message: {}", err);
        }
    }

    #[test]
    fn test_establish_pool_success() {
        dotenv().ok();
        let pool = establish_pool();
        assert!(pool.is_ok(), "Expected pool creation to succeed, but it failed.");
    }

    #[test]
    fn test_establish_pool_invalid_database_url() {
        dotenv().ok();
        env::set_var("DATABASE_URL", "invalid_path"); // Simulate invalid URL

        let pool = establish_pool();
        assert!(pool.is_err(), "Expected pool creation to fail, but it succeeded.");

        if let Err(FastingAppError::ConnectionError(err)) = pool {
            assert!(err.contains("Failed to create connection pool"), "Unexpected error message: {}", err);
        }
    }

    #[test]
    fn test_get_connection_success() {
        dotenv().ok();
        let pool = establish_pool().expect("Failed to create pool");
        let connection = get_connection(&pool);
        assert!(connection.is_ok(), "Expected to get a connection, but it failed.");
    }

    #[test]
    fn test_get_connection_pool_exhausted() {
        dotenv().ok();
        let pool = establish_pool().expect("Failed to create pool");

        // Simulate pool exhaustion by trying to retrieve more connections than allowed
        let max_connections = 10; // Adjust this to match your pool's configured size
        let mut connections = Vec::new();
        for _ in 0..max_connections {
            let conn = get_connection(&pool);
            assert!(conn.is_ok(), "Expected to get a connection, but it failed before the pool exhausted.");
            connections.push(conn);
        }

        // Now the pool should be exhausted
        let connection = get_connection(&pool);
        assert!(connection.is_err(), "Expected connection retrieval to fail when pool is exhausted.");
    }
}
