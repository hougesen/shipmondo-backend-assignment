use base64::Engine;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::shipmondo::error::ShipmondoError;

pub(crate) mod error;

#[inline]
fn encode_auth_header(user: &str, key: &str) -> String {
    base64::prelude::BASE64_URL_SAFE.encode(format!("{user}:{key}"))
}

pub struct Shipmondo(reqwest::Client);

impl Shipmondo {
    #[inline]
    fn api_url(&self) -> &'static str {
        if self.production {
            "https://app.shipmondo.com/api/public/v3"
        } else {
            "https://sandbox.shipmondo.com/api/public/v3"
        }
    }

    #[inline]
    pub fn new(auth: String, production: bool) -> Result<Self, ShipmondoError> {
        let auth_header_value =
            HeaderValue::try_from(auth).map_err(ShipmondoError::InvalidAuthorizationHeaderValue)?;

        let headers =
            HeaderMap::from_iter([(HeaderName::from_static("Authorization"), auth_header_value)]);

        let http_client = reqwest::ClientBuilder::new()
            // Not really necessary, but I like to tell external apis who I am
            .default_headers(headers)
            .user_agent("MadsHougesen +http://mhouge.dk")
            .build()
            .map_err(ShipmondoError::InitializeHttpClient)?;

        Ok(Shipmondo(http_client))
    }
}
