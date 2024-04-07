use std::env;

pub mod terminal;
pub mod envloader;

pub enum AppType {
    Interactive,
    Command(env::Args),
}

pub fn determine_app_type() -> AppType {
    let command_arguments = env::args();
    if command_arguments.len() > 1 {
        return AppType::Command(command_arguments);
    }
    AppType::Interactive
}
