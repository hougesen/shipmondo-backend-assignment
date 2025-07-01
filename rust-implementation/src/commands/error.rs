use crate::commands::add_user::error::AddUserError;
use crate::commands::remove_user::error::RemoveUserError;

#[derive(Debug)]
pub enum CliError {
    AddUser(AddUserError),
    RemoveUser(RemoveUserError),
}

impl core::fmt::Display for CliError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddUser(error) => error.fmt(f),
            Self::RemoveUser(error) => error.fmt(f),
        }
    }
}

impl core::error::Error for CliError {}
