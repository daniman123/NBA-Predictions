use read_write::{config_reader::Config, read_df_from_json, write_df_to_json};

/// # Arguments
/// - `input_save_path`: Input path`&str` to `DataFrame` stored in json file.
/// - `output_save_path`: Output path`&str` to store `DataFrame`, as json file.
/// - `columns_to_be_removed`: Vector `Vec<&str>` containing the names of `DataFrame` columns to be removed.
/// # Returns
/// - `()`: void.
///
pub fn drop_unused_columns_and_store_df(
    input_save_path: &str,
    output_save_path: &str,
    columns_to_be_removed: Vec<&str>,
) {
    let mut df = read_df_from_json(input_save_path);
    df = df.drop_many(&columns_to_be_removed);
    write_df_to_json(output_save_path, df);
}

pub fn iterate_through_datasets(config: Config, container_columns_to_be_removed: &[Vec<&str>; 5]) {
    for (index, input_json_save_rnd_2) in config.json_data_save_paths_round_2.iter().enumerate() {
        let output_json_save_rnd_3 = &config.json_data_save_paths_round_3[index];
        drop_unused_columns_and_store_df(
            input_json_save_rnd_2,
            output_json_save_rnd_3,
            container_columns_to_be_removed[index].clone(),
        );
    }
}

pub fn round_3_processing_dataframe() {
    let config = Config::new();

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

    iterate_through_datasets(config, container_columns_to_be_removed);
}
