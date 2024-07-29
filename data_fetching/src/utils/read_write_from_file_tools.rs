use crate::Result;
use std::{ fs::{ self, File }, io::Read, path::PathBuf };

/// Writes the given contents to a file at the specified path.
///
/// # Arguments
///
/// * `path` - A path where the file should be written. Can be any type that converts to `PathBuf`.
/// * `contents` - The content to be written to the file as bytes.
///
/// # Returns
///
/// * `Result<()>` - Returns an `Ok(())` on success, or an error if the operation fails.
pub fn write_to_file<P, C>(path: P, contents: C) -> Result<()>
    where P: Into<PathBuf>, C: AsRef<[u8]>
{
    fs::write(path.into(), contents.as_ref()).map_err(Into::into)
}

/// Reads all bytes from a file at the given path.
///
/// # Arguments
///
/// * `path` - A type that can be converted into a `PathBuf`, representing the file path.
///
/// # Returns
///
/// * `Result<Vec<u8>>` - A `Result` containing a vector of bytes if successful, or an `io::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the file does not exist or the process lacks permissions to open the file.
///
/// # Examples
///
/// ```
/// let bytes = read_bytes_from_file("example.txt")?;
/// ```
pub fn read_bytes_from_file<P>(path: P) -> Result<Vec<u8>> where P: Into<PathBuf> {
    let mut file = File::open(path.into())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_write_to_file() {
        let path = "../data/raw_response_data/test_output.bin";
        let contents = b"Hello, Rust!";

        // Write to file
        write_to_file(path, contents).expect("Failed to write to file");

        // Verify the file content
        let mut file = fs::File::open(path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        assert_eq!(buffer, contents);

        // Clean up
        fs::remove_file(path).expect("Failed to delete test file");
    }

    #[test]
    fn test_read_bytes_from_file() {
        let path = "../data/raw_response_data/test_output.bin";
        let conversion_results = read_bytes_from_file(path);
        assert!(conversion_results.is_ok())
    }
}
