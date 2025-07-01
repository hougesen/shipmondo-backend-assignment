#[derive(Debug)]
pub enum AddUserError {
    DatabaseConnection(diesel::ConnectionError),
    Inquire(inquire::InquireError),
    InsertNewUser(diesel::result::Error),
}

impl core::fmt::Display for AddUserError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseConnection(error) => error.fmt(f),
            Self::Inquire(error) => error.fmt(f),
            Self::InsertNewUser(error) => error.fmt(f),
        }
    }
}

impl core::error::Error for AddUserError {}
