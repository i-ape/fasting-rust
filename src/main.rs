use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Register { username: String, password: String },
    Login { username: String, password: String },
    StartFasting { user_id: i32 },
    StopFasting { session_id: i32 },
}

fn main() {
    let args = Cli::from_args();
    match args.command {
        Command::Register { username, password } => {
            // Call the registration function here
        }
        Command::Login { username, password } => {
            // Call the login function here
        }
        Command::StartFasting { user_id } => {
            // Call the start fasting function here
        }
        Command::StopFasting { session_id } => {
            // Call the stop fasting function here
        }
    }
}
