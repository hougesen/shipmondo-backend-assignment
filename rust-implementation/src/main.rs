mod commands;
pub mod database;
pub mod error;
mod logging;
mod models;
mod schema;
// mod shipmondo;

fn main() {
    if let Err(error) = commands::run_cli() {
        logging::log_error(&error);

        std::process::exit(1);
    }
}
