mod error;
mod response_data_extractors;
mod utils;

pub use self::error::{Error, Result};
use read_write::{config_reader::Config, serde_json_writer, write_bytes_to_file};
use response_data_extractors::{
    nba_api_result_sets_extractor::extract_result_sets_json_data,
    rotowire_series_extractor::extract_rotowire_series_data,
};
use utils::{
    fetch_request::{fetch, get_response_as_json},
    req_headers::custom_headers,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();

    let api_endpoints = [
        &config.api_endpoints.team_stats_advanced,
        &config.api_endpoints.opponent_shooting,
        &config.api_endpoints.team_stats_base,
        &config.api_endpoints.player_averages,
        &config.api_endpoints.injury_report,
    ];

    for (index, api_endpoint) in api_endpoints.iter().enumerate() {
        let raw_data_bytes_save_path = &config.raw_data_save_paths[index];
        let json_save_path = &config.json_data_save_paths[index];
        let json_save_path_round_2 = &config.json_data_save_paths_round_2[index];

        let headers = custom_headers()?;
        let response = fetch(api_endpoint.to_owned(), Some(headers)).await?;
        let response_body_bytes = response.bytes().await?;
        write_bytes_to_file(raw_data_bytes_save_path, response_body_bytes.clone());
        let response_body_json = get_response_as_json(response_body_bytes)?;
        // Store raw json response body
        serde_json_writer(json_save_path, response_body_json.clone())?;

        // Extract data in raw response json body and store data
        if index == 4 {
            // If data is rotowire api
            extract_rotowire_series_data(response_body_json, json_save_path_round_2)?
        } else {
            // If data is nba api result sets
            extract_result_sets_json_data(response_body_json, json_save_path_round_2)?
        }
    }

    Ok(())
}
