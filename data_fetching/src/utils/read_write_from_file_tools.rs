use crate::Result;
use std::{fs, path::PathBuf};

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
where
    P: Into<PathBuf>,
    C: AsRef<[u8]>,
{
    fs::write(path.into(), contents.as_ref()).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_write_to_file() {
        let path = "../data/raw_response_data/test_output.txt";
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
}
