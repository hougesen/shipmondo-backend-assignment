#[derive(Debug)]
pub enum ShipmondoError {
    InvalidAuthorizationHeaderValue(reqwest::header::InvalidHeaderValue),
    InitializeHttpClient(reqwest::Error),
}

impl std::error::Error for ShipmondoError {}

impl core::fmt::Display for ShipmondoError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAuthorizationHeaderValue(error) => {
                write!(f, "Invalid authorization header value - {error}")
            }
            Self::InitializeHttpClient(error) => {
                write!(f, "Error initializing Shipmondo http client - {error}")
            }
        }
    }
}
