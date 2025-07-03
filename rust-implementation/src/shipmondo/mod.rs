use base64::Engine;
use reqwest::header::{HeaderName, HeaderValue, InvalidHeaderValue};

use crate::error::CliError;

#[inline]
fn encode_auth_header(user: &str, key: &str) -> String {
    base64::prelude::BASE64_URL_SAFE.encode(format!("{user}:{key}"))
}

#[derive(serde::Deserialize)]
pub struct CreateShipmentResponseBodyParcel {
    pub pkg_no: Option<String>,

    pub pkg_nos: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
pub struct CreateShipmentResponseBody {
    pub id: i32,

    pub price: String,

    pub parcels: Vec<CreateShipmentResponseBodyParcel>,
}

#[inline]
const fn api_url(production: bool) -> &'static str {
    if production {
        "https://app.shipmondo.com/api/public/v3"
    } else {
        "https://sandbox.shipmondo.com/api/public/v3"
    }
}

#[inline]
fn auth_headers(
    username: &str,
    password: &str,
) -> Result<(HeaderName, HeaderValue), InvalidHeaderValue> {
    let auth_header_value =
        HeaderValue::from_str(&format!("Basic {}", encode_auth_header(username, password)))?;

    Ok((HeaderName::from_static("authorization"), auth_header_value))
}

#[inline]
pub fn create_shipment(
    username: &str,
    password: &str,
    production: bool,
) -> Result<CreateShipmentResponseBody, CliError> {
    let (auth_header_name, auth_header_value) = auth_headers(username, password)?;

    let response = reqwest::blocking::Client::new()
        .post(format!("{}/shipments", api_url(production)))
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        )
        .header(auth_header_name, auth_header_value)
        .body(mock_shipment_body())
        .send()?
        .error_for_status()?;

    let contents = response.json::<CreateShipmentResponseBody>()?;

    Ok(contents)
}

#[derive(serde::Deserialize)]
pub struct GetBalanceResponseBody {
    pub amount: f32,

    pub currency_code: String,
}

#[inline]
pub fn get_balance(
    username: &str,
    password: &str,
    production: bool,
) -> Result<GetBalanceResponseBody, CliError> {
    let (auth_header_name, auth_header_value) = auth_headers(username, password)?;

    let response = reqwest::blocking::Client::new()
        .get(format!("{}/account/balance", api_url(production)))
        .header(auth_header_name, auth_header_value)
        .send()?
        .error_for_status()?;

    let contents = response.json::<GetBalanceResponseBody>()?;

    Ok(contents)
}

const fn mock_shipment_body() -> &'static str {
    r#"{
  "own_agreement": false,
  "label_format": "a4_pdf",
  "product_code": "GLSDK_SD",
  "service_codes": "EMAIL_NT,SMS_NT",
  "reference": "Order 10001",
  "automatic_select_service_point": true,
  "sender": {
    "name": "ToTypeOrNotToType",
    "attention": "Mads Hougesen",
    "address1": "Hvilehøjvej 25",
    "address2": null,
    "zipcode": "5220",
    "city": "Odense SØ",
    "country_code": "DK",
    "email": "mads@mhouge.dk",
    "mobile": "70400407"
  },
  "receiver": {
    "name": "Mads Hougesen",
    "attention": null,
    "address1": "Skibhusvej 52",
    "address2": null,
    "zipcode": "5000",
    "city": "Odense C",
    "country_code": "DK",
    "email": "lene@email.dk",
    "mobile": "12345678"
  },
  "parcels": [
    {
      "weight": 1000
    }
  ]
}"#
}
