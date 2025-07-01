mod commands;
pub mod database;
mod logging;
mod models;
mod schema;
// mod shipmondo;

fn main() {
    if let Err(error) = commands::run_cli() {
        eprintln!("{error}");
    }
}
