use read_write::{config_reader::Config, read_df_from_json, write_df_to_json};

pub fn round_3_processing_dataframe() {
    let config = Config::new();
    let save_path_round_2_teams_general_advanced = &config.json_data_save_paths[0];
    let save_path_round_2_opponent_shooting_general = &config.json_data_save_paths[1];
    let save_path_round_2_teams_general_opponent = &config.json_data_save_paths[2];
    let save_path_round_2_players_general = &config.json_data_save_paths[3];

    let mut df_teams_general_advanced = read_df_from_json(save_path_round_2_teams_general_advanced);
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
    df_teams_general_advanced =
        df_teams_general_advanced.drop_many(&cols_to_remove_df_teams_general_advanced);
    // Store df
    let save_path_round_3_teams_general_advanced = &config.json_data_save_paths_round_3[0];
    write_df_to_json(
        save_path_round_3_teams_general_advanced,
        df_teams_general_advanced,
    );

    let mut df_opponent_shooting_general =
        read_df_from_json(save_path_round_2_opponent_shooting_general);
    let cols_to_remove_df_opponent_shooting_general = ["G"];

    df_opponent_shooting_general =
        df_opponent_shooting_general.drop_many(&cols_to_remove_df_opponent_shooting_general);
    // Store df
    let save_path_round_3_opponent_shooting_general = &config.json_data_save_paths_round_3[1];
    write_df_to_json(
        save_path_round_3_opponent_shooting_general,
        df_opponent_shooting_general,
    );

    let mut df_teams_general_opponent = read_df_from_json(save_path_round_2_teams_general_opponent);
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

    df_teams_general_opponent =
        df_teams_general_opponent.drop_many(&cols_to_remove_df_teams_general_opponent);
    // Store df
    let save_path_round_3_teams_general_opponent = &config.json_data_save_paths_round_3[2];
    write_df_to_json(
        save_path_round_3_teams_general_opponent,
        df_teams_general_opponent,
    );

    let mut df_players_general = read_df_from_json(save_path_round_2_players_general);
    // println!("{:?}", df_players_general.get_column_names());
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

    df_players_general = df_players_general.drop_many(&cols_to_remove_df_players_general);
    // Store df
    let save_path_round_3_players_general = &config.json_data_save_paths_round_3[3];
    write_df_to_json(save_path_round_3_players_general, df_players_general);
}
