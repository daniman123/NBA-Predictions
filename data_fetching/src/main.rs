mod error;
mod utils;

pub use self::error::{Error, Result};
use read_write::{config_reader::Config, serde_json_writer, write_bytes_to_file};
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

        let headers = custom_headers()?;
        let response = fetch(api_endpoint.to_owned(), Some(headers)).await?;
        let response_body_bytes = response.bytes().await?;
        write_bytes_to_file(raw_data_bytes_save_path, response_body_bytes.clone());
        let response_body_json = get_response_as_json(response_body_bytes)?;
        // Store raw json response body
        serde_json_writer(json_save_path, response_body_json)?;
    }

    Ok(())
}
