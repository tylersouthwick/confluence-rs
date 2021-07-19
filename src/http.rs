//! HTTP helpers.

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use crate::Error as HttpError;
pub use reqwest::StatusCode;
use std::io::Read;
use std::result;

/// Simplified HTTP response representation.
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
}

/// Perform a GET request to specified URL.
pub fn get(client : &reqwest::Client, url: &str) -> Result<Response> {
    let mut res = client.get(url).send()?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let status = res.status();

    Ok(Response { status, body })
}

/// Perform a SOAP action to specified URL.
pub fn soap_action(client : &reqwest::Client, url: &str, action: &str, xml: &str) -> Result<Response> {
    let soap_action = HeaderName::from_bytes(b"SOAPAction").unwrap();
    let soap_value = HeaderValue::from_str(action).unwrap();
    let mut hmap = HeaderMap::new();
    hmap.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse().unwrap());
    hmap.insert(soap_action, soap_value);

    let mut response = client
        .post(url)
        .headers(hmap)
        .body(xml.to_string())
        .send()?;

    if response.status() != 200 {
        return Err(HttpError::InvalidStatusCode(response.status()));
    }
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    let status = response.status();

    Ok(Response { status, body })
}

pub type Result<T> = result::Result<T, HttpError>;
