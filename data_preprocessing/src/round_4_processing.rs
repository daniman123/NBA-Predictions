use read_write::{config_reader::Config, read_df_from_json, write_df_to_json};

pub fn round_4_processing_dataframe() {
    let config = Config::new();
    let save_path_round_3_teams_general_advanced = &config.json_data_save_paths_round_3[0];
    let save_path_round_3_opponent_shooting_general = &config.json_data_save_paths_round_3[1];
    let save_path_round_3_teams_general_opponent = &config.json_data_save_paths_round_3[2];

    let mut df_teams_general_advanced = read_df_from_json(save_path_round_3_teams_general_advanced);
    let cols_to_remove_df_teams_general_advanced = ["TEAM_ID","PACE", "DEF_RATING"];
    df_teams_general_advanced = df_teams_general_advanced
        .select(cols_to_remove_df_teams_general_advanced)
        .unwrap();
    // Store df
    let save_path_round_4_teams_general_advanced = &config.json_data_save_paths_round_4[0];
    write_df_to_json(
        save_path_round_4_teams_general_advanced,
        df_teams_general_advanced,
    );

    let mut df_opponent_shooting_general =
        read_df_from_json(save_path_round_3_opponent_shooting_general);
    let cols_to_remove_df_opponent_shooting_general = ["TEAM_ID","FG2_PCT", "FG3_PCT"];

    df_opponent_shooting_general = df_opponent_shooting_general
        .select(cols_to_remove_df_opponent_shooting_general)
        .unwrap();
    // Store df
    let save_path_round_4_teams_general_advanced = &config.json_data_save_paths_round_4[1];
    write_df_to_json(
        save_path_round_4_teams_general_advanced,
        df_opponent_shooting_general,
    );


    let mut df_teams_general_opponent = read_df_from_json(save_path_round_3_teams_general_opponent);
    let cols_to_remove_df_teams_general_opponent = ["TEAM_ID","FT_PCT"];

    df_teams_general_opponent = df_teams_general_opponent
        .select(cols_to_remove_df_teams_general_opponent)
        .unwrap();
    // Store df
    let save_path_round_4_teams_general_advanced = &config.json_data_save_paths_round_4[2];
    write_df_to_json(
        save_path_round_4_teams_general_advanced,
        df_teams_general_opponent,
    );
}
