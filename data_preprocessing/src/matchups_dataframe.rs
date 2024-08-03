use polars::{frame::UniqueKeepStrategy, prelude::{DataFrameJoinOps, JoinArgs}};
use read_write::{read_df_from_json, write_df_to_json, Config};

/// Creates a DataFrame of game matchups from game logs and writes it to a JSON file.
///
/// This function performs the following steps:
/// 1. Reads the game log data from a JSON file.
/// 2. Drops unnecessary columns from the DataFrame.
/// 3. Sorts the DataFrame by the `GAME_ID` column.
/// 4. Extracts unique rows based on the `GAME_ID` column, keeping the first and last occurrences.
/// 5. Joins these unique rows to create a DataFrame of game matchups.
/// 6. Writes the resulting DataFrame to a JSON file.
///
/// # Panics
/// This function will panic if any of the following operations fail:
/// - Reading the DataFrame from JSON.
/// - Dropping columns.
/// - Sorting the DataFrame.
/// - Extracting unique rows.
/// - Joining DataFrames.
/// - Writing the DataFrame to JSON.
///
/// # Examples
/// ```
/// create_matchups_df_from_game_logs();
/// ```
pub fn create_matchups_df_from_game_logs() {
    // Path to the input JSON file
    let config = Config::new();
    // let path = "../data/json_data/for_preprocessing_test_output.json";
    
    // Read the DataFrame from the JSON file
    let mut df = read_df_from_json(&config.input_path);

    // Drop unnecessary columns from the DataFrame
    df = df.drop_many(&["PLUS_MINUS", "VIDEO_AVAILABLE"]);
    
    // Sort the DataFrame by the `GAME_ID` column
    df = df.sort(["GAME_ID"], Default::default()).unwrap();

    // Extract unique rows based on the `GAME_ID` column, keeping the first occurrences
    let unique_game_id_first = df
        .unique(Some(&["GAME_ID".to_owned()]), UniqueKeepStrategy::First, None)
        .unwrap();
    
    // Extract unique rows based on the `GAME_ID` column, keeping the last occurrences
    let unique_game_id_last = df
        .unique(Some(&["GAME_ID".to_owned()]), UniqueKeepStrategy::Last, None)
        .unwrap();

    // Join the unique rows to create a DataFrame of game matchups
    let games = unique_game_id_first
        .join(&unique_game_id_last, ["GAME_ID"], ["GAME_ID"], JoinArgs::default())
        .unwrap();

    // Path to the output JSON file
    // let path = "../data/processed_data/json_data/game_matchups.json";
    
    // Write the resulting DataFrame to the JSON file
    write_df_to_json(&config.output_path, games);
}
