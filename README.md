
#  Fasting-Rust 🕰️🚀

A fasting tracker built with Rust, using Diesel, SQLite, and bcrypt for authentication and data management.

🛠️ Tech Stack

Rust 🦀

Diesel ORM (for database interactions)

SQLite (lightweight database)

bcrypt (password hashing)

dotenv (environment variables)

cargo-expand (macro expansion for debugging)



---

📂 Project Structure

├── analytics.rs       # Core analytics functions (history, streaks, etc.)
├── auth.rs            # Authentication-related utilities
├── db.rs              # Database connection setup
├── errors.rs          # Custom error handling
├── export.rs          # (Planned) Export fasting data
├── handlers/          # Business logic handlers
│   ├── analytics.rs   # Handles fasting analytics  
│   ├── fasting.rs     # Manages fasting sessions  
│   ├── mod.rs         # Module handler  
├── lib.rs             # Main library module  
├── main.rs            # Entry point of the application  
├── models.rs          # Database models  
├── notifications.rs   # (Planned) User notifications  
├── schema.rs          # Diesel-generated schema  
├── temp_handlers.rs   # (Temporary) Placeholder for testing features  
├── users/             # User-related functionality  
│   ├── create.rs      # User registration  
│   ├── find.rs        # Find users by ID, username, or device ID  
│   ├── login.rs       # User authentication  
│   ├── mod.rs         # User module handler  
│   ├── update.rs      # User profile updates  
└── utils.rs           # (Planned) Utility functions


---

✅ Features Implemented

✔ User Authentication

Create new users

Secure password hashing with bcrypt

Login via username/password or device ID


✔ Fasting Management

Start/stop fasting sessions

Track ongoing and past fasting events

Support for fasting goals


✔ Analytics

View fasting history

Calculate average fasting duration

Calculate total fasting time


✔ Error Handling

Custom error types for better debugging



---

🚧 Work in Progress

🔄 Fasting Goals System

Allow modifying fasting goals mid-fast

View past fasting goals


🔄 Notifications System

Reminders to start/stop fasting

Alerts for missed goals


🔄 Export Data

Generate CSV/JSON of fasting history


🔄 Improved Testing

Implement unit tests and integration tests


🔄 Improve CLI/UI

Make command-line menus more interactive

Support mobile-friendly usage



---

🐛 Current Issues & Debugging

Some functions aren't being used (find.rs, users/mod.rs)

"Black hole" functions (code running without expected output)

Cargo warns about unused functions & imports

Need integration tests before scaling further



---

📌 Next Steps

1️⃣ Fix user-related functions (ensure all find functions are correctly used)
2️⃣ Improve fasting goal handling (allow editing/deleting goals mid-fast)
3️⃣ Start writing tests with cargo test & criterion.rs
4️⃣ Implement user notifications (via CLI alerts)
5️⃣ Prepare for mobile integration (test go branch for performance)


---

🛠️ Developer Notes

Running cargo expand can debug macro issues

Diesel schema updates require running migrations:

diesel migration redo

Working on mobile? Use GitHub Mobile for pull requests & merges



---

📣 Contributing & Feedback

Found a bug? Submit an issue 📌

Want to add a feature? Fork & create a PR ✨

Need help? Ping me on GitHub! 🚀



---

This README clearly documents your project’s progress, making it easier for collaborators (or future you) to jump in quickly! 🚀 Let me know if you want any adjustments!

