use polars::prelude::*;
use read_write::{config_reader::Config, read_df_from_json, write_df_to_json};

pub fn round_5_processing_dataframe() {
    let config = Config::new();
    let save_path_round_4_teams_general_advanced = &config.json_data_save_paths_round_4[0];
    let save_path_round_4_opponent_shooting_general = &config.json_data_save_paths_round_4[1];
    let save_path_round_4_teams_general_opponent = &config.json_data_save_paths_round_4[2];

    let df_teams_general_advanced = read_df_from_json(save_path_round_4_teams_general_advanced);
    let df_opponent_shooting_general =
        read_df_from_json(save_path_round_4_opponent_shooting_general);
    let df_teams_general_opponent = read_df_from_json(save_path_round_4_teams_general_opponent);

    let mut final_team_stats = df_teams_general_advanced
        .join(
            &df_opponent_shooting_general,
            ["TEAM_ID"],
            ["TEAM_ID"],
            JoinArgs::default(),
        )
        .unwrap();

    final_team_stats = final_team_stats
        .join(
            &df_teams_general_opponent,
            ["TEAM_ID"],
            ["TEAM_ID"],
            JoinArgs::default(),
        )
        .unwrap();

    let out = final_team_stats
        .clone()
        .lazy()
        .select([
            col("*").exclude(["TEAM_ID"]),
            col("TEAM_ID").cast(DataType::Int32),
        ])
        .collect()
        .unwrap();

    // Store df
    let save_path_round_5_teams_general_opponent = &config.json_data_save_paths_round_5[0];
    write_df_to_json(save_path_round_5_teams_general_opponent, out);

    // Players
    let save_path_round_4_players_general = &config.json_data_save_paths_round_4[3];
    let mut df_players_general = read_df_from_json(save_path_round_4_players_general);

    df_players_general = df_players_general
        .clone()
        .lazy()
        .select([
            (col("FGA") - col("FG3A")).alias("FG2A"),
            (col("FGM") - col("FG3M")).alias("FG2M"),
            col("*"),
        ])
        .collect()
        .unwrap();

    // Store df
    let save_path_round_5_players_general = &config.json_data_save_paths_round_5[1];
    write_df_to_json(save_path_round_5_players_general, df_players_general);
}
