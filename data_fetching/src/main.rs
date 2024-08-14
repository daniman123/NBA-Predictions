mod error;
mod response_data_extractors;
mod utils;

pub use self::error::{Error, Result};
use read_write::config_reader::Config;
use response_data_extractors::nba_api_result_sets_extractor::extract_result_sets_json_data;
use utils::{fetch_request::fetch, req_headers::custom_headers};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();

    let api_endpoints = [
        &config.api_endpoints.team_stats_advanced,
        &config.api_endpoints.opponent_shooting,
        &config.api_endpoints.team_stats_base,
        &config.api_endpoints.player_averages,
    ];

    for (index, api_endpoint) in api_endpoints.iter().enumerate() {
        let json_save_path = &config.json_data_save_paths[index];

        let headers = custom_headers()?;
        let response = fetch(*api_endpoint, Some(headers)).await?;
        let response_body = response.json().await?;
        extract_result_sets_json_data(response_body, json_save_path)?
    }

    Ok(())
}
