//! HTTP helpers.

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use crate::Error as HttpError;
pub use reqwest::StatusCode;
use std::result;
use reqwest::Client;

/// Simplified HTTP response representation.
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
}

/// Perform a GET request to specified URL.
pub async fn get(client : &Client, url: &str) -> Result<Response> {
    let res = client.get(url).send().await?;
    let status = res.status();
    let body = res.text().await?;

    Ok(Response { status, body })
}

/// Perform a SOAP action to specified URL.
pub async fn soap_action(client : &Client, url: &str, action: &str, xml: &str) -> Result<Response> {
    let soap_action = HeaderName::from_bytes(b"SOAPAction").unwrap();
    let soap_value = HeaderValue::from_str(action).unwrap();
    let mut hmap = HeaderMap::new();
    hmap.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse().unwrap());
    hmap.insert(soap_action, soap_value);

    let response = client
        .post(url)
        .headers(hmap)
        .body(xml.to_string())
        .send()
        .await?;

    if response.status() != 200 {
        return Err(HttpError::InvalidStatusCode(response.status()));
    }
    let status = response.status();
    let body = response.text().await?;

    Ok(Response { status, body })
}

pub type Result<T> = result::Result<T, HttpError>;
