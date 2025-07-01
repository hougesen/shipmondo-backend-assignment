use clap::{Parser, Subcommand};

use crate::{database::connect_to_database, error::CliError};

mod add_user;
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
    let mut database = connect_to_database()?;

    match Cli::parse().command {
        CliCommand::AddUser => add_user::command(&mut database),
        CliCommand::RemoveUser => remove_user::command(&mut database),
    }
}
