mod error;
pub mod decompressed_json_reader;
pub mod game_log_json_data_into_data_frame;
pub mod utils;
mod fetch_endpoint_response;

use decompressed_json_reader::read_decompressed_json;
use fetch_endpoint_response::get_response_bytes;
use game_log_json_data_into_data_frame::read_json_to_df;
use read_write::config_reader::Config;
use utils::read_write_from_file_tools::write_json_to_file;
use std::{ fs::File, io::Read };
pub use self::error::{ Error, Result };

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = {
        let mut file = File::open("../data/config.json").expect("Config file not found");
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read config file");
        serde_json::from_str(&data).expect("Failed to parse config file")
    };

    let api_endpoints = [
        &config.api_endpoints.team_stats_advanced,
        &config.api_endpoints.opponent_shooting,
        &config.api_endpoints.team_stats_base,
    ];

    for (index, api_endpoint) in api_endpoints.iter().enumerate() {
        let bin_save_path = &config.bin_data_save_paths[index];
        let json_save_path = &config.json_data_save_paths[index];
        let json_save_path_round_2 = &config.json_data_save_paths_round_2[index];

        get_response_bytes(api_endpoint, bin_save_path).await?;
        let json_data = read_decompressed_json(bin_save_path).unwrap();
        write_json_to_file(json_save_path, json_data).unwrap();
        read_json_to_df(json_save_path, json_save_path_round_2);
    }

    Ok(())
}
