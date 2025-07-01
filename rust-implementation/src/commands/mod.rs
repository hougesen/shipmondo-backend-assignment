use clap::{Parser, Subcommand};

use crate::{
    commands::{add_user::error::AddUserError, error::CliError},
    database::connect_to_database,
};

mod add_user;
mod error;
mod remove_user;

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, propagate_version = true, help_template = HELP_TEMPLATE)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    AddUser,

    RemoveUser,
}

pub fn run_cli() -> Result<(), CliError> {
    match Cli::parse().command {
        CliCommand::AddUser => {
            let mut database = connect_to_database()
                .map_err(AddUserError::DatabaseConnection)
                .map_err(CliError::AddUser)?;

            add_user::command(&mut database).map_err(CliError::AddUser)
        }
        CliCommand::RemoveUser => remove_user::command().map_err(CliError::RemoveUser),
    }
}
