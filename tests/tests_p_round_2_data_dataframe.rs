use read_write::read_df_from_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_2_teams_general_advanced_data() {
        let path =
        "../data/round_2_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json";
        let mut df_teams_general_advanced = read_df_from_json(path);

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
        println!("{:?}", df_teams_general_advanced.get_column_names());
    }

    #[test]
    fn test_round_2_opponent_shooting_general_data() {
        let path =
        "../data/round_2_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json";
        let mut df_opponent_shooting_general = read_df_from_json(path);

        let cols_to_remove_df_opponent_shooting_general = ["G"];

        df_opponent_shooting_general =
            df_opponent_shooting_general.drop_many(&cols_to_remove_df_opponent_shooting_general);
        println!("{:?}", df_opponent_shooting_general.get_column_names());
    }

    #[test]
    fn test_round_2_teams_general_opponent_data() {
        let path =
        "../data/round_2_data/json_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.json";
        let mut df_teams_general_opponent = read_df_from_json(path);

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
        println!("{:?}", df_teams_general_opponent.get_column_names());
    }

    #[test]
    fn test_round_2_players_general_data() {
        let path =
        "../data/round_2_data/json_data/player_stats/players_general/per_game/2023_24/players_general_per_game_2023_24.json";
        let mut df_players_general = read_df_from_json(path);
        println!("{:?}", df_players_general.get_column_names());

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
        println!("{:?}", df_players_general.get_column_names());
    }
}
