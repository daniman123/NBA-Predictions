use crate::Result;
use flate2::read::GzDecoder;
use reqwest::{header::HeaderMap, Client, Response};
use serde_json::Value;
use std::io::Read;

/// Wrapper function for extracting response body as json
/// 
/// This function gets the response body as bytes and decompresses it,
/// according to encoding, as stated in response header. This wrapper function should work
/// broadly for all response bodies irregardless of api used (within this project.).
/// 
/// # Arguments
/// - `response`: The 'Response' from the fetch request.
/// 
/// # Returns
/// - `Ok(Value)`: The response body as json object.
/// - `Err(serde_json::Error)`: If there is an error during the response handling.
pub async fn get_response_as_json(response: Response) -> Result<Value> {
    let response_body_bytes = response.bytes().await?;
    let mut d = GzDecoder::new(&response_body_bytes[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    let response_body_json = serde_json::from_str::<Value>(&s)?;
    Ok(response_body_json)
}

/// Fetches a URL with optional headers asynchronously.
///
/// This function sends a GET request to the specified URL using the `reqwest` client,
/// applying the given headers if provided. It returns the server's response as a `Response` object.
///
/// # Arguments
/// - `url`: The URL to send the request to. Can be converted into a `String`.
/// - `headers`: An optional `HeaderMap` containing headers to include in the request.
///
/// # Returns
/// - `Ok(Response)`: The response from the server if the request is successful.
/// - `Err(reqwest::Error)`: If there is an error during the request or response handling.
///
/// # Example
/// ```rust,no_run
/// use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
///
/// #[tokio::main]
/// async fn main() {
///     let mut headers = HeaderMap::new();
///     headers.insert(USER_AGENT, HeaderValue::from_static("example-agent"));
///
///     match fetch("https://example.com", Some(headers)).await {
///         Ok(response) => println!("Success: {:?}", response),
///         Err(e) => eprintln!("Error: {:?}", e),
///     }
/// }
/// ```
pub async fn fetch(url: impl Into<String>, headers: Option<HeaderMap>) -> Result<Response> {
    let client = Client::new();
    let mut request = client.get(url.into());

    if let Some(h) = headers {
        request = request.headers(h);
    }

    let response = request.send().await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

    #[tokio::test]
    async fn test_fetch_with_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("example-agent"));

        let result = fetch("https://httpbin.org/get", Some(headers)).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        println!("{:?}", response.headers());
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_fetch_without_headers() {
        let result = fetch("https://httpbin.org/get", None).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.status().is_success());
    }
}
