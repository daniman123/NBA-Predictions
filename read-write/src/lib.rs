pub mod config_reader;
use polars::prelude::*;
use std::{env, io::Result, path::{Path, PathBuf}};

fn create_parent_dir_if_needed<P>(file_path: P) -> Result<()> where P: Into<PathBuf> {
    if let Some(parent) = Path::new(&file_path.into()).parent() {
        std::fs::create_dir_all(parent)?;
    }
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
