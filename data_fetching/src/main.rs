mod error;
mod response_data_extractors;
mod utils;

use std::io::Read;

pub use self::error::{Error, Result};
use flate2::read::GzDecoder;
use read_write::{config_reader::Config, serde_json_writer};
use response_data_extractors::{
    nba_api_result_sets_extractor::extract_result_sets_json_data,
    rotowire_series_extractor::extract_rotowire_series_data,
};
// use serde_json::Value;
use utils::{fetch_request::fetch, req_headers::custom_headers};

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
        let raw_data_json_save_path = &config.raw_data_save_paths[index];
        let json_save_path = &config.json_data_save_paths[index];

        // println!("api_endpoint: {:?}", *api_endpoint);
        let headers = custom_headers()?;
        let response = fetch(api_endpoint.to_owned(), Some(headers)).await?;
        // println!("response: {:?}", response);
        let response_body_bytes = response.bytes().await?;

        // println!("response_body_json: {:?}", response_body_bytes);
        let mut d = GzDecoder::new(&response_body_bytes[..]);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();
        let response_body_json: serde_json::Value = serde_json::from_str(&s)?;

        serde_json_writer(raw_data_json_save_path, response_body_json.clone())?;

        // Extract data in raw response json body and store data
        if index == 4 {
            // If data is rotowire api
            extract_rotowire_series_data(response_body_json, json_save_path)?
        } else {
            // If data is nba api result sets
            extract_result_sets_json_data(response_body_json, json_save_path)?
        }
    }

    Ok(())
}
