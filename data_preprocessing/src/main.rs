pub mod error;
pub mod matchups_dataframe;
pub mod response_data_extractors;
// mod round_2_processing;
// mod round_3_processing;
// mod round_4_processing;
mod round_5_processing;

pub use self::error::{Error, Result};
use read_write::{config_reader::Config, serde_json_reader, write_df_to_json};
use response_data_extractors::{
    nba_api_result_sets_extractor::extract_result_sets_json_data,
    rotowire_series_extractor::extract_rotowire_series_data,
};
// use round_2_processing::round_2_processing_dataframe;
// use round_3_processing::round_3_processing_dataframe;
// use round_4_processing::round_4_processing_dataframe;
use round_5_processing::round_5_processing_dataframe;

fn main() -> Result<()> {
    let config: Config = Config::new();

    let cols_to_remove_df_teams_general_advanced = [
        "PACE_RANK",
        "MIN_RANK",
        "AST_TO_RANK",
        "NET_RATING_RANK",
        "W_RANK",
        "TM_TOV_PCT_RANK",
        "AST_PCT_RANK",
        "OREB_PCT_RANK",
        "TS_PCT_RANK",
        "EFG_PCT_RANK",
        "GP_RANK",
        "W_PCT_RANK",
        "AST_RATIO_RANK",
        "PIE_RANK",
        "DEF_RATING_RANK",
        "L_RANK",
        "OFF_RATING_RANK",
        "DREB_PCT_RANK",
        "REB_PCT_RANK",
    ];
    let cols_to_remove_df_opponent_shooting_general = ["G"];
    let cols_to_remove_df_teams_general_opponent = [
        "MIN_RANK",
        "STL_RANK",
        "FG_PCT_RANK",
        "FG3M_RANK",
        "OREB_RANK",
        "W_RANK",
        "DREB_RANK",
        "BLK_RANK",
        "FT_PCT_RANK",
        "BLKA_RANK",
        "PTS_RANK",
        "GP_RANK",
        "W_PCT_RANK",
        "FGM_RANK",
        "FGA_RANK",
        "L_RANK",
        "FG3A_RANK",
        "FTM_RANK",
        "FTA_RANK",
        "REB_RANK",
        "FG3_PCT_RANK",
        "TOV_RANK",
        "PFD_RANK",
        "PLUS_MINUS_RANK",
        "AST_RANK",
        "PF_RANK",
    ];
    let cols_to_remove_df_players_general = [
        "BLKA_RANK",
        "FGM_RANK",
        "STL_RANK",
        "FTA_RANK",
        "L_RANK",
        "W_RANK",
        "FG3_PCT_RANK",
        "DD2_RANK",
        "OREB_RANK",
        "MIN_RANK",
        "AST_RANK",
        "FT_PCT_RANK",
        "FG3A_RANK",
        "FG_PCT_RANK",
        "GP_RANK",
        "NBA_FANTASY_PTS_RANK",
        "FG3M_RANK",
        "BLK_RANK",
        "TOV_RANK",
        "PF_RANK",
        "WNBA_FANTASY_PTS_RANK",
        "PLUS_MINUS_RANK",
        "W_PCT_RANK",
        "FGA_RANK",
        "FTM_RANK",
        "PTS_RANK",
        "TD3_RANK",
        "DREB_RANK",
        "REB_RANK",
        "PFD_RANK",
        "NBA_FANTASY_PTS",
        "WNBA_FANTASY_PTS",
        "TD3",
        "NICKNAME",
        "PLUS_MINUS",
        "DD2",
    ];
    let cols_to_remove_df_injury_report = ["ID", "URL", "r_date"];
    let container_columns_to_be_removed: &[Vec<&str>; 5] = &[
        cols_to_remove_df_teams_general_advanced.to_vec(),
        cols_to_remove_df_opponent_shooting_general.to_vec(),
        cols_to_remove_df_teams_general_opponent.to_vec(),
        cols_to_remove_df_players_general.to_vec(),
        cols_to_remove_df_injury_report.to_vec(),
    ];

    let cols_to_keep_df_teams_general_advanced = ["TEAM_ID", "PACE", "DEF_RATING"];
    let cols_to_keep_df_opponent_shooting_general = ["TEAM_ID", "FG2_PCT", "FG3_PCT"];
    let cols_to_keep_df_teams_general_opponent = ["TEAM_ID", "FT_PCT"];
    let cols_to_keep_df_players_general = [
        "PTS",
        "FTM",
        "FG3M",
        "FT_PCT",
        "FGA",
        "FG3A",
        "TEAM_ID",
        "PLAYER_ID",
        "FG_PCT",
        "FTA",
        "MIN",
        "FGM",
        "GP",
        "PLAYER_NAME",
        "FG3_PCT",
    ];
    let cols_to_keep_df_injury_report = ["player", "team", "position", "status"];

    let container_columns_to_keep: &[Vec<&str>; 5] = &[
        cols_to_keep_df_teams_general_advanced.to_vec(),
        cols_to_keep_df_opponent_shooting_general.to_vec(),
        cols_to_keep_df_teams_general_opponent.to_vec(),
        cols_to_keep_df_players_general.to_vec(),
        cols_to_keep_df_injury_report.to_vec(),
    ];

    for (index, rnd_1_save_path) in config.json_data_save_paths.iter().enumerate() {
        // Save paths
        let json_save_path_round_2 = &config.json_data_save_paths_round_2[index];
        let json_save_path_round_3 = &config.json_data_save_paths_round_3[index];
        let json_save_path_round_4 = &config.json_data_save_paths_round_4[index];

        // Round 2
        // Extract data in raw response json body and store data
        let response_body_json = serde_json_reader(rnd_1_save_path)?;
        let mut df = if index == 4 {
            // If data is rotowire api
            extract_rotowire_series_data(response_body_json, json_save_path_round_2)?
        } else {
            // If data is nba api result sets
            extract_result_sets_json_data(response_body_json, json_save_path_round_2)?
        };

        // Round 3
        df = df.drop_many(&container_columns_to_be_removed[index]);
        write_df_to_json(json_save_path_round_3, df.clone());

        // Round 4
        df = df.select(&container_columns_to_keep[index]).unwrap();
        write_df_to_json(json_save_path_round_4, df.clone());
    }

    // round_2_processing_dataframe()?;
    // round_3_processing_dataframe();
    // round_4_processing_dataframe();
    round_5_processing_dataframe();
    Ok(())
}
