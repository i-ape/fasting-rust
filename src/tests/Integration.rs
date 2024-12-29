#[cfg(test)]
mod tests {
    use super::*;
    use diesel::Connection;

    #[test]
    fn test_register_user() {
        let mut conn = establish_test_connection(); // Mocked or test connection
        let result = register_user(&mut conn, "testuser", "testpassword");
        assert!(result.is_ok(), "User registration failed: {:?}", result);
    }
}
