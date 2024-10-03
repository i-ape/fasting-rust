use crate::db::establish_connection;

pub fn create_user(username: &str, password: &str) {
    println!("Creating user: {} with password: {}", username, password);
    let _conn = establish_connection();
    // Insert user logic here
}

pub fn start_fasting(user_id: i32) {
    println!("Starting fasting session for user ID: {}", user_id);
}

pub fn stop_fasting(session_id: i32) {
    println!("Stopping fasting session for session ID: {}", session_id);
}
