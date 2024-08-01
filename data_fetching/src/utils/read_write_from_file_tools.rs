use crate::Result;
use std::{ fs::{ self, File }, io::{ BufWriter, Read, Write }, path::PathBuf };

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

/// Writes the given JSON contents to the specified file path.
///
/// # Arguments
///
/// * `path` - A value that can be converted into a `PathBuf`, representing the file path where the JSON contents will be written.
/// * `contents` - A `serde_json::Value` containing the JSON data to be written to the file.
///
/// # Returns
///
/// This function returns a `Result<()>`:
/// * `Ok(())` if the JSON contents are successfully written to the file.
/// * An `Err` variant if an error occurs during the file opening, writing, or flushing operations.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just these cases:
/// * The file does not exist or the user lacks permissions to open it for writing.
/// * There is insufficient space on the disk to write the JSON contents.
/// * An I/O error occurs during the writing or flushing process.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("output.json");
/// let contents = json!({ "key": "value" });
///
/// match write_json_to_file(path, contents) {
///     Ok(_) => println!("JSON contents successfully written to file."),
///     Err(e) => eprintln!("Failed to write JSON to file: {}", e),
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the JSON serialization fails. This is unlikely if the `serde_json::Value` is well-formed.
pub fn write_json_to_file<P>(path: P, contents: serde_json::Value) -> Result<()>
where
    P: Into<PathBuf>
{
    let file = File::create(path.into())?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &contents).unwrap();
    writer.flush()?;
    Ok(())
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
    use serde_json::json;

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

    #[test]
    fn test_write_json_to_file() {
        // let path = "a.json";
        let path = "../data/json_data/test_output.json";
        let json_data = json!({"name":"testName","age":123});

        let write_json_result = write_json_to_file(path, json_data);
        assert!(write_json_result.is_ok())
    }
}
