use std::path::PathBuf;
use crate::Result;
use crate::utils::{
    process_bytes::decompress_bytes_to_string,
    read_write_from_file_tools::read_bytes_from_file,
};

/// Reads a compressed file, decompresses its contents, and parses it as JSON.
///
/// # Arguments
///
/// * `path` - A path to the compressed file.
///
/// # Returns
///
/// A `Result` containing the parsed JSON data (`serde_json::Value`) on success, or an error on failure.
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be read.
/// - The data cannot be decompressed.
/// - The decompressed data cannot be parsed as JSON.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use crate::read_decompressed_json;
///
/// let json_data = read_decompressed_json(PathBuf::from("path/to/compressed/file")).expect("Failed to read and parse JSON");
/// ```
pub fn read_decompressed_json<P>(path: P) -> Result<serde_json::Value> where P: Into<PathBuf> {
    let bytes = read_bytes_from_file(path)?;
    // Convert stored bytes to string
    let string_data = decompress_bytes_to_string(&bytes)?;

    let json_data = serde_json::from_str::<serde_json::Value>(&string_data)?;
    Ok(json_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_decompressed_json() {
        let path = "../data/raw_response_data/test_output.bin";
        let json_data = read_decompressed_json(path);
        assert!(json_data.is_ok())
    }
}
