use std::io::Read;

use crate::Result;
use reqwest::Response;
use bytes::Bytes;
use flate2::read::GzDecoder;

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

/// Decompresses Gzip-compressed bytes to a string.
///
/// # Arguments
///
/// * `compressed_bytes` - A slice of compressed bytes.
///
/// # Returns
///
/// * `Result<String, io::Error>` - A `Result` containing the decompressed string if successful, or an `io::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the decompression fails.
pub fn decompress_bytes_to_string(compressed_bytes: &[u8]) -> Result<String> {
    let mut decoder = GzDecoder::new(compressed_bytes);
    let mut decompressed_string = String::new();
    decoder.read_to_string(&mut decompressed_string)?;
    Ok(decompressed_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    /// Compresses a string to Gzip-compressed bytes.
    fn compress_string_to_bytes(input: &str) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(input.as_bytes()).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn test_decompress_bytes_to_string() {
        let original_string = "Hello, world!";
        let compressed_bytes = compress_string_to_bytes(original_string);
        let decompressed_string = decompress_bytes_to_string(&compressed_bytes).unwrap();
        assert_eq!(original_string, decompressed_string);
    }

    #[test]
    fn test_decompress_empty_bytes() {
        let original_string = "";
        let compressed_bytes = compress_string_to_bytes(original_string);
        let decompressed_string = decompress_bytes_to_string(&compressed_bytes).unwrap();
        assert_eq!(original_string, decompressed_string);
    }

    #[test]
    fn test_decompress_long_string() {
        let original_string = "The quick brown fox jumps over the lazy dog. ".repeat(100);
        let compressed_bytes = compress_string_to_bytes(&original_string);
        let decompressed_string = decompress_bytes_to_string(&compressed_bytes).unwrap();
        assert_eq!(original_string, decompressed_string);
    }

    #[test]
    fn test_decompress_invalid_bytes() {
        let invalid_bytes = vec![0, 1, 2, 3, 4, 5];
        let result = decompress_bytes_to_string(&invalid_bytes);
        assert!(result.is_err());
    }
}