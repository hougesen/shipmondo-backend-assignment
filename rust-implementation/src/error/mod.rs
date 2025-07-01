use core::num::ParseFloatError;

#[derive(Debug)]
pub enum CliError {
    Database(diesel::result::Error),
    DatabaseConnection(diesel::ConnectionError),
    Inquire(inquire::InquireError),
    NoUsersFound,
    Reqwest(reqwest::Error),
    HeaderValue(reqwest::header::InvalidHeaderValue),
    SerdeJson(serde_json::Error),
    ParseFloat(ParseFloatError),
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
            Self::Reqwest(error) => error.fmt(f),
            Self::HeaderValue(error) => error.fmt(f),
            Self::SerdeJson(error) => error.fmt(f),
            Self::ParseFloat(error) => error.fmt(f),
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

impl From<reqwest::Error> for CliError {
    #[inline]
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for CliError {
    #[inline]
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        Self::HeaderValue(value)
    }
}

impl From<serde_json::Error> for CliError {
    #[inline]
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<ParseFloatError> for CliError {
    #[inline]
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloat(value)
    }
}
