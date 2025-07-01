#[derive(Debug)]
pub enum CliError {
    Database(diesel::result::Error),
    DatabaseConnection(diesel::ConnectionError),
    Inquire(inquire::InquireError),
    NoUsersFound,
}

impl core::fmt::Display for CliError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Database(error) => error.fmt(f),
            Self::DatabaseConnection(error) => error.fmt(f),
            Self::Inquire(error) => error.fmt(f),
            Self::NoUsersFound => write!(
                f,
                "No users have been added. Run the add-user command to add a new user"
            ),
        }
    }
}

impl core::error::Error for CliError {}

impl From<inquire::InquireError> for CliError {
    #[inline]
    fn from(value: inquire::InquireError) -> Self {
        Self::Inquire(value)
    }
}

impl From<diesel::result::Error> for CliError {
    #[inline]
    fn from(value: diesel::result::Error) -> Self {
        Self::Database(value)
    }
}

impl From<diesel::ConnectionError> for CliError {
    #[inline]
    fn from(value: diesel::ConnectionError) -> Self {
        Self::DatabaseConnection(value)
    }
}
