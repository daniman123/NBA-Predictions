use crate::response_data_extractors::{
    nba_api_result_sets_extractor::extract_result_sets_json_data,
    rotowire_series_extractor::extract_rotowire_series_data,
};
use crate::Result;
use read_write::config_reader::Config;
use read_write::serde_json_reader;

pub fn round_2_processing_dataframe() -> Result<()> {
    let config = Config::new();
    let api_endpoints = [
        &config.api_endpoints.team_stats_advanced,
        &config.api_endpoints.opponent_shooting,
        &config.api_endpoints.team_stats_base,
        &config.api_endpoints.player_averages,
        &config.api_endpoints.injury_report,
    ];

    for (index, _) in api_endpoints.iter().enumerate() {
        let json_save_path = &config.json_data_save_paths[index];
        let json_save_path_round_2 = &config.json_data_save_paths_round_2[index];

        // Extract data in raw response json body and store data
        if index == 4 {
            let response_body_json = serde_json_reader(json_save_path)?;
            // If data is rotowire api
            extract_rotowire_series_data(response_body_json, json_save_path_round_2)?
        } else {
            let response_body_json = serde_json_reader(json_save_path)?;
            // If data is nba api result sets
            extract_result_sets_json_data(response_body_json, json_save_path_round_2)?
        }
    }
    Ok(())
}
