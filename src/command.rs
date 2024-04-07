use std::env;

pub fn run_command(args: env::Args) {
    println!("Running in command mode with args: {:?}", args);
}
