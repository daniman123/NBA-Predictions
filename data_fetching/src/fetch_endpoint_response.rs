/// This module provides functionality to fetch response bytes from an API endpoint
/// and save them to a specified file path.

use crate::Result;
use crate::utils::{
    read_write_from_file_tools::write_to_file,
    fetch_request::fetch,
    process_bytes::extract_bytes_from_response,
    req_headers::custom_headers,
};

/// Fetches the response bytes from the given API endpoint and saves them to a file.
///
/// This function performs the following steps:
/// 1. Creates custom headers for the request.
/// 2. Sends a fetch request to the specified API endpoint with the custom headers.
/// 3. Extracts the raw bytes from the fetch response.
/// 4. Writes the extracted bytes to the specified file path.
///
/// # Arguments
///
/// * `api_endpoint` - A string slice that holds the URL of the API endpoint to fetch the response from.
/// * `bytes_data_save_path` - A string slice that holds the file path where the fetched bytes will be saved.
///
/// # Returns
///
/// This function returns a `Result<()>` which is:
/// * `Ok(())` if the operation is successful.
/// * `Err` if any step of the operation fails.
///
/// # Errors
///
/// This function will return an error if:
/// * Custom headers creation fails.
/// * The fetch request fails.
/// * Extracting bytes from the response fails.
/// * Writing the bytes to the file fails.
///
/// # Examples
///
/// ```rust
/// use crate::utils::get_response_bytes;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let api_endpoint = "https://api.example.com/data";
///     let save_path = "data/response_bytes.bin";
///     get_response_bytes(api_endpoint, save_path).await?;
///     Ok(())
/// }
/// ```
pub async fn get_response_bytes(api_endpoint: &str, bytes_data_save_path: &str) -> Result<()> {
    let headers = custom_headers()?;

    let response = fetch(api_endpoint, Some(headers)).await;

    let raw_bytes_data = extract_bytes_from_response(response).await?;

    write_to_file(bytes_data_save_path, raw_bytes_data)
}
