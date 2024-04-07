mod command;
mod interactive;
mod todo;
mod util;

fn main() {
    util::envloader::load_env();
    match util::determine_app_type() {
        util::AppType::Interactive => interactive::run_interactive(),
        util::AppType::Command(args) => command::run_command(args),
    }
}
