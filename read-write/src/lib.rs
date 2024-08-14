pub mod config_reader;
use polars::prelude::*;
use std::{env, fs::File, io::{BufWriter, Result, Write}, path::{Path, PathBuf}};

pub fn create_parent_dir_if_needed<P>(file_path: P) -> Result<()> where P: Into<PathBuf> {
    if let Some(parent) = Path::new(&file_path.into()).parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
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
/// match serde_json_writer(path, contents) {
///     Ok(_) => println!("JSON contents successfully written to file."),
///     Err(e) => eprintln!("Failed to write JSON to file: {}", e),
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the JSON serialization fails. This is unlikely if the `serde_json::Value` is well-formed.
pub fn serde_json_writer<P>(path: P, contents: serde_json::Value) -> Result<()>
    where P: Into<PathBuf> + std::marker::Copy
{
    create_parent_dir_if_needed(path).unwrap();
    let file = File::create(path.into())?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &contents).unwrap();
    writer.flush()?;
    Ok(())
}

pub fn read_df_from_json(path: &str) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    JsonReader::new(&mut file).finish().unwrap()
}

pub fn write_df_to_json(path: &str, mut df: DataFrame) {
    create_parent_dir_if_needed(path).unwrap();
    let mut file = std::fs::File::create(path).unwrap();
    // json
    JsonWriter::new(&mut file)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)
        .unwrap();
}

#[derive(Default)]
pub struct Config {
    pub input_path: String,
    pub output_path: String,
}

impl Config {
    pub fn new() -> Self {
        let input_path = env::var("INPUT_PATH")
            .unwrap_or_else(|_| "../data/json_data/for_preprocessing_test_output.json".to_string());
        let output_path = env::var("OUTPUT_PATH")
            .unwrap_or_else(|_| "../data/processed_data/json_data/game_matchups.json".to_string());

        Config {
            input_path,
            output_path,
        }
    }
}
