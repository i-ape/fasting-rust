# fasting-rust

with this app my goal is to make a full application for fasting

this is the largest of the apps i have seen and i hope it goes well :)

## tools i used for this

ill add more as i think of them
chatgpt
vscode
diesel
sqlite
cheats.rs rust info mainly for importing crates and other documentation

currently i have the issues with variables and having consistant info in them between all the files, next i must debug and proceed,


├── analytics.rs
├── auth.rs
├── db.rs
├── errors.rs
├── export.rs
├── handlers
│   ├── analytics.rs
│   ├── fasting.rs
│   └── mod.rs
├── lib.rs
├── main.rs
├── models.rs
├── notifications.rs
├── schema.rs
├── temp_handlers.rs
├── users
│   ├── create.rs
│   ├── find.rs
│   ├── login.rs
│   ├── mod.rs
│   └── update.rs
└── utils.rs

this is my current structure
have issues with fn in users either not being used or a weird blocks where code seems to go into black holes and in terminal its says 1 issue, next I want to add user goals and history sections although it may be too early 

did some reading I should do integration tests and do a
cargo install cargo-expand
cargo expand
https://bheisler.github.io/criterion.rs/book/index.html supposedly a good src
must find a way to do merges and pull requests from mobile