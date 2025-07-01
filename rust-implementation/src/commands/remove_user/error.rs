#[derive(Debug)]
pub enum RemoveUserError {
    Inquire(inquire::InquireError),
}

impl core::fmt::Display for RemoveUserError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inquire(error) => error.fmt(f),
        }
    }
}

impl core::error::Error for RemoveUserError {}
