use read_write::read_df_from_json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_4_teams_general_advanced_data() {
        let path =
        "../data/round_3_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json";
        let mut df_teams_general_advanced = read_df_from_json(path);
        // println!("{:?}", df_teams_general_advanced.get_column_names());

        let cols_to_remove_df_teams_general_advanced = ["TEAM_ID","PACE", "DEF_RATING"];
        df_teams_general_advanced = df_teams_general_advanced
            .select(cols_to_remove_df_teams_general_advanced)
            .unwrap();

        println!("{:?}", df_teams_general_advanced.get_column_names());
    }

    #[test]
    fn test_round_4_opponent_shooting_general_data() {
        let path =
        "../data/round_3_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json";
        let mut df_opponent_shooting_general = read_df_from_json(path);
        // println!("{:?}", df_opponent_shooting_general.get_column_names());

        let cols_to_remove_df_opponent_shooting_general = ["TEAM_ID","FG2_PCT","FG3_PCT"];

        df_opponent_shooting_general = df_opponent_shooting_general
            .select(cols_to_remove_df_opponent_shooting_general)
            .unwrap();
        println!("{:?}", df_opponent_shooting_general.get_column_names());
    }

    #[test]
    fn test_round_4_teams_general_opponent_data() {
        let path =
        "../data/round_3_data/json_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.json";
        let mut df_teams_general_opponent = read_df_from_json(path);
        // println!("{:?}", df_teams_general_opponent.get_column_names());

        let cols_to_remove_df_teams_general_opponent = ["TEAM_ID","FT_PCT"];

        df_teams_general_opponent = df_teams_general_opponent
            .select(cols_to_remove_df_teams_general_opponent)
            .unwrap();
        println!("{:?}", df_teams_general_opponent.get_column_names());
    }
}
