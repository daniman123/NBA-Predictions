use crate::Result;
use reqwest::Response;
use bytes::Bytes;

/// Extracts the body as bytes from a given HTTP response.
///
/// # Arguments
///
/// * `response` - A `Result` containing an HTTP `Response`.
///
/// # Returns
///
/// * `Result<Bytes>` - On success, returns the response body as `Bytes`.
///   On failure, propagates the error from the `response`.
///
/// # Errors
///
/// This function will return an error if:
/// - The `response` is an `Err`.
/// - The conversion to bytes fails.
///
/// # Examples
///
/// ```
/// # use reqwest::Response;
/// # use bytes::Bytes;
/// # async fn example(response: Result<Response, reqwest::Error>) {
/// let bytes = extract_bytes_from_response(response).await;
/// match bytes {
///     Ok(data) => println!("Received data: {:?}", data),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// # }
/// ```
pub async fn extract_bytes_from_response(response: Result<Response>) -> Result<Bytes> {
    match response {
        Ok(response) => {
            let bytes = response.bytes().await?;
            Ok(bytes)
        }
        Err(e) => { Err(e) }
    }
}

