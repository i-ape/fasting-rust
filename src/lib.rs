pub mod errors;
pub mod handlers; // Declares `handlers/mod.rs` and its submodules
pub mod models;
pub mod schema;
pub mod users; // Declare `users/mod.rs` and its submodules

pub use handlers::{start_fasting, stop_fasting}; // From `handlers/mod.rs`
pub use users::{create_user, login_user, register_user}; // From `users/mod.rs`
